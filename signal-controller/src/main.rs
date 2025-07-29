use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

lazy_static! {
    static ref LAST_EXECUTION: Mutex<SystemTime> = Mutex::new(SystemTime::now());
}

const ADDR: &str = "0.0.0.0:50051";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(ADDR).await?;
    println!("🚦 信号机服务启动，监听 50051...");

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
