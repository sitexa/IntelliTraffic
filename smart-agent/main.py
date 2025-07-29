import asyncio
import json
import warnings

import numpy as np
import torch
from stable_baselines3 import DQN, A2C, PPO, SAC

# 从配置文件导入参数
from config import (
    ALGO_NAME, MODEL_PATH, INITIAL_PHASE, MIN_GREEN_TIME, YELLOW_DURATION,
    CONTROLLER_HOST, CONTROLLER_PORT, AGENT_HOST, AGENT_PORT,
    PHASE_TO_LIGHTS, PHASE_TRANSITIONS, PHASE_SEQUENCE
)

# 忽略stable-baselines3的警告
warnings.filterwarnings('ignore', category=UserWarning, module='stable_baselines3')

# 算法映射字典
ALGO_MAP = {
    'DQN': DQN,
    'A2C': A2C,
    'PPO': PPO,
    'SAC': SAC
}

def load_model_with_algo_check():
    """根据配置加载模型并验证算法匹配性"""
    print(f"🤖 配置算法: {ALGO_NAME}")
    print(f"📁 模型路径: {MODEL_PATH}")
    
    # 检查算法是否支持
    if ALGO_NAME not in ALGO_MAP:
        raise ValueError(f"不支持的算法: {ALGO_NAME}，支持的算法: {list(ALGO_MAP.keys())}")
    
    # 获取对应的算法类
    algo_class = ALGO_MAP[ALGO_NAME]
    
    try:
        # 加载模型
        model = algo_class.load(MODEL_PATH, custom_objects={
            'lr_schedule': lambda x: 0.0001,
            'exploration_schedule': lambda x: 0.1
        })
        
        # 验证模型类型与配置算法是否匹配
        if type(model).__name__ != ALGO_NAME:
            raise ValueError(f"模型算法类型 {type(model).__name__} 与配置算法 {ALGO_NAME} 不匹配")
        
        print(f"✅ 模型加载成功，算法匹配验证通过")
        print(f"📊 模型信息: {type(model).__name__}")
        
        return model
        
    except Exception as e:
        print(f"❌ 模型加载失败: {e}")
        raise

# 加载强化学习模型
model = load_model_with_algo_check()

# 信号机状态参数
current_phase = INITIAL_PHASE  # 当前相位
current_phase_duration = 0  # 当前相位持续时长(秒)
min_green = [0]  # 满足最小绿：1；未满足最小绿:0


# 添加时间管理类
class PhaseTimer:
    def __init__(self):
        try:
            loop = asyncio.get_running_loop()
        except RuntimeError:
            loop = asyncio.new_event_loop()
            asyncio.set_event_loop(loop)
        self.phase_start_time = loop.time()
        self.current_phase = 0

    def get_duration(self):
        try:
            loop = asyncio.get_running_loop()
        except RuntimeError:
            loop = asyncio.get_event_loop()
        current_time = loop.time()
        return int(current_time - self.phase_start_time)

    def reset(self):
        try:
            loop = asyncio.get_running_loop()
        except RuntimeError:
            loop = asyncio.get_event_loop()
        self.phase_start_time = loop.time()


# 创建全局定时器实例
phase_timer = PhaseTimer()


async def handle_detector(reader, writer):
    """处理雷视机发送的交通状态信息"""
    global current_phase, current_phase_duration, min_green

    while True:
        try:
            data = await reader.read(1024)
            if not data:
                break

            # 固定处理间隔
            await asyncio.sleep(1.0)  # 每秒处理一次数据

            # 获取当前相位持续时间
            current_phase_duration = phase_timer.get_duration()
            # print(f"当前相位: {current_phase}, 持续时间: {current_phase_duration}秒")

            # 如果当前相位是黄灯相位，则跳过决策
            if current_phase not in PHASE_SEQUENCE:
                continue
                
            # 解析交通状态数据
            traffic_state = json.loads(data.decode())
            
            # 智能判断状态数据格式并构造强化学习状态输入
            if len(traffic_state) == 43:  # 完整状态向量（4相位+1最小绿+38交通数据）
                # 直接使用完整的状态向量
                state = np.array(traffic_state, dtype=np.float32)
            elif len(traffic_state) == 38:  # 仅包含交通数据（19密度+19排队长度）
                # 需要添加相位和最小绿灯信息
                phase_one_hot = [1 if i == current_phase else 0 for i in range(4)]
                min_green = [0 if current_phase_duration <= MIN_GREEN_TIME else 1]
                state = np.concatenate([
                    phase_one_hot,  # 当前相位(one-hot编码)
                    min_green,  # 是否满足最小绿
                    traffic_state  # 交通状态数据
                ])
            else:
                print(f"⚠️ 未知的状态向量长度: {len(traffic_state)}，跳过处理")
                continue

            # 模型推理
            state_tensor = torch.tensor(state, dtype=torch.float32).unsqueeze(0)
            action = int(model.predict(state_tensor, deterministic=True)[0])
            # print("状态state:", state)
            # print(f"动作action:{action}")
            # todo 推理出的相位，是绿灯相位: 0,1,2,3
            # todo 相位切换时，需要增加黄灯阶段过度: 0-4,1-5,2-6,3-7.

            await switch_phase(action)

        except Exception as e:
            print(f"处理数据错误: {e}")
            break

    writer.close()


async def switch_phase(action):
    """切换信号相位
    Args:
        action: 目标相位
    """
    global current_phase, current_phase_duration, min_green

    print(f"当前相位:{current_phase} ({current_phase_duration}s){min_green} => 目标相位:{action} ")

    # 判断当前相位和目标相位的类型
    is_current_green = current_phase in PHASE_SEQUENCE
    is_target_green = action in PHASE_SEQUENCE

    if not is_current_green or not is_target_green:
        return

    if action == current_phase:
        return

    # 如果当前相位运行时间小于最小绿灯时间，则不允许切换相位
    if current_phase_duration < MIN_GREEN_TIME:
        print(f"当前相位未满足最小绿灯时间({current_phase_duration}/{MIN_GREEN_TIME}秒)，不切换")
        return

    # 从绿灯切换到另一个绿灯相位时，需要黄灯过渡
    yellow_phase = PHASE_TRANSITIONS[current_phase]
    await send_signal_command(yellow_phase)
    
    # 更新当前相位为黄灯相位，但不重置计时器
    current_phase = yellow_phase
    
    # 等待黄灯时间
    print(f"等待黄灯时间: {YELLOW_DURATION}秒")
    await asyncio.sleep(YELLOW_DURATION)

    # 发送目标相位指令
    await send_signal_command(action)

    # 重置计时器并更新状态
    phase_timer.reset()
    current_phase_duration = 0
    current_phase = action
    min_green = [0]


async def send_signal_command(action):
    """向信号机发送信号控制指令"""
    try:
        reader, writer = await asyncio.open_connection(CONTROLLER_HOST, CONTROLLER_PORT)
        command = PHASE_TO_LIGHTS.get(action)
        print("目标灯色:", command)

        writer.write(command.encode())
        await writer.drain()

        response = await reader.read(1024)
        if response.decode().strip('"') == "SUCCESS":
            print(f"🚦 信号切换成功: {action}")
        else:
            print(f"❌ 信号切换失败")

        writer.close()
        await writer.wait_closed()

    except Exception as e:
        print(f"发送控制指令错误: {e}")


async def main():
    """启动智能体服务"""
    server = await asyncio.start_server(
        handle_detector,
        AGENT_HOST,
        AGENT_PORT
    )
    print("🧠 智能体启动，监听 50052...")

    async with server:
        await server.serve_forever()


if __name__ == "__main__":
    asyncio.run(main())
