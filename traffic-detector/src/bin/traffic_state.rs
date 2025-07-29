use chrono::Utc;
use std::time::Duration;
use tokio::time::sleep;

fn generate_traffic_state(t: f32) -> Vec<f32> {
    let traffic_state = vec![];
    let lane_count = 19;
    let density: Vec<f32> = (0..lane_count)
        .map(|i| (((t + i as f32 * 0.1).sin() * 0.4 + 0.5) * 100.0).round() / 100.0 as f32) // 生成平滑的密度曲线，保留2位小数
        .collect();

    let queue_length: Vec<f32> = (0..lane_count)
        .map(|i| (((t + i as f32 * 0.2).cos() * 0.4 + 0.5) * 100.0).round() / 100.0 as f32) // 生成平滑的排队长度曲线，保留2位小数
        .collect();

    traffic_state.into_iter()
        .chain(density)
        .chain(queue_length)
        .collect()
}

#[tokio::main]
async fn main() {
    let mut t = 0.0;
    loop {
        let state_vector = generate_traffic_state(t);
        println!("{}: \n{:?}", Utc::now(), state_vector);
        t += 0.1;
        sleep(Duration::from_secs(3)).await;
    }
}
