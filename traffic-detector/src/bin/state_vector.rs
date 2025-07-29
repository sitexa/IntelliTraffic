use chrono::Utc;
use std::time::Duration;
use tokio::time::sleep;

fn generate_state_vector(t: f32) -> Vec<f32> {
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
async fn main() {
    let mut t = 0.0;
    loop {
        let state_vector = generate_state_vector(t);
        println!("{}: \n{:?}", Utc::now(), state_vector);
        t += 0.1;
        sleep(Duration::from_secs(3)).await;
    }
}
