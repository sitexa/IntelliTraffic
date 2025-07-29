use std::path::Path;
use std::time::Duration;
use config::{Config, File as ConfigFile, FileFormat};
use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug, Deserialize)]
struct AppConfig {
    agent_host: String,
    agent_port: u16,
}

fn load_config() -> Result<AppConfig, config::ConfigError> {
    let config_path = "config.toml";

    // 检查配置文件是否存在，如果不存在则创建默认配置
    if !Path::new(config_path).exists() {
        println!("⚠️ 配置文件不存在，使用默认配置");
        let default_config = AppConfig {
            agent_host: "0.0.0.0".to_string(),
            agent_port: 50052,
        };
        return Ok(default_config);
    }

    let config = Config::builder()
        .add_source(ConfigFile::new(config_path, FileFormat::Toml))
        .build()?;

    config.try_deserialize::<AppConfig>()
}

fn generate_traffic_state(t: f32) -> Vec<f32> {
    // 根据时间计算当前相位，每30秒切换一次相位
    // t每0.1递增，程序每3秒输出一次，所以t*30对应实际秒数
    let current_time_seconds = (t * 30.0) as u32;
    let phase_index = (current_time_seconds / 30) % 4; // 每30秒切换相位，4个相位循环
    
    let mut phase_one_hot = vec![0.0, 0.0, 0.0, 0.0];
    phase_one_hot[phase_index as usize] = 1.0; // 设置当前相位为1.0
    
    // 计算当前相位已持续的时间（秒）
    let time_in_current_phase = current_time_seconds % 30;
    // 如果当前相位已持续超过10秒，则最小绿灯时间已满足
    let min_green = if time_in_current_phase >= 10 { vec![1.0] } else { vec![0.0] };

    let lane_count = 19;
    let density: Vec<f32> = (0..lane_count)
        .map(|i| (((t + i as f32 * 0.1).sin() * 0.4 + 0.5) * 100.0).round() / 100.0 as f32) // 生成平滑的密度曲线，保留2位小数
        .collect();

    let queue_length: Vec<f32> = (0..lane_count)
        .map(|i| (((t + i as f32 * 0.2).cos() * 0.4 + 0.5) * 100.0).round() / 100.0 as f32) // 生成平滑的排队长度曲线，保留2位小数
        .collect();

    phase_one_hot.into_iter()
        .chain(min_green)
        .chain(density)
        .chain(queue_length)
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = load_config()?;

    // 组合 AGENT_ADDR
    let agent_addr = format!("{}:{}", config.agent_host, config.agent_port);
    println!("🔧 使用配置: {}", agent_addr);

    let mut t = 0.0;
    loop {
        // 尝试连接到服务器
        match TcpStream::connect(&agent_addr).await {
            Ok(mut stream) => {
                println!("🎥 已连接到智能代理 {}", agent_addr);

                // 模拟交通数据采集
                let traffic_state = generate_traffic_state(t);
                t += 0.1;

                // 发送数据
                match serde_json::to_string(&traffic_state) {
                    Ok(data) => {
                        if let Err(e) = stream.write_all(data.as_bytes()).await {
                            println!("❌ 发送数据失败: {}", e);
                            continue;
                        }
                        println!("📤 已发送交通状态数据:\n{:?}", data);
                    }
                    Err(e) => {
                        println!("❌ 序列化数据失败: {}", e);
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("❌ 连接失败: {}", e);
                println!("🔄 3秒后重试连接...");
            }
        }

        tokio::time::sleep(Duration::from_secs(3)).await;
    }
}