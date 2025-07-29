use lazy_static::lazy_static;
use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use config::{Config, File as ConfigFile, FileFormat};
use serde::Deserialize;

lazy_static! {
    static ref LAST_EXECUTION: Mutex<SystemTime> = Mutex::new(SystemTime::now());
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    controller_host: String,
    controller_port: u16,
}

fn load_config() -> Result<AppConfig, config::ConfigError> {
    let config_path = "config.toml";

    // 检查配置文件是否存在，如果不存在则创建默认配置
    if !Path::new(config_path).exists() {
        println!("⚠️ 配置文件不存在，使用默认配置");
        let default_config = AppConfig {
            controller_host: "0.0.0.0".to_string(),
            controller_port: 50051,
        };
        return Ok(default_config);
    }

    let config = Config::builder()
        .add_source(ConfigFile::new(config_path, FileFormat::Toml))
        .build()?;

    config.try_deserialize::<AppConfig>()
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = load_config()?;

    // 组合地址
    let addr = format!("{}:{}", config.controller_host, config.controller_port);
    println!("🔧 使用配置: {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    println!("🚦 信号机服务启动，监听 {}...", config.controller_port);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            match socket.read(&mut buffer).await {
                Ok(n) if n == 0 => return, // 连接关闭
                Ok(n) => {
                    if let Ok(light_colors) = std::str::from_utf8(&buffer[..n]) {
                        let now = SystemTime::now();
                        let last = *LAST_EXECUTION.lock().unwrap();
                        let duration = now.duration_since(last).unwrap_or(Duration::from_secs(0));
                        println!("📥 收到控制指令:{} (上条指令: {:.2}秒)", light_colors.trim(), duration.as_secs_f64());
                        *LAST_EXECUTION.lock().unwrap() = now;
                        // 调用CAN总线接口，发送驱动命令

                        // 发送确认消息
                        let response = serde_json::to_string(&"SUCCESS").unwrap();
                        if let Err(e) = socket.write_all(response.as_bytes()).await {
                            eprintln!("发送响应失败: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("读取失败: {}", e),
            }
        });
    }
}
