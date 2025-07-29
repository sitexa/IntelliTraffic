# 强化学习模型配置
ALGO_NAME="DQN"
MODEL_PATH = "./zszx-2-model-DQN.zip"

# 信号机状态参数
INITIAL_PHASE = 0  # 初始相位
MIN_GREEN_TIME = 10  # 最小绿灯时长(秒)
YELLOW_DURATION = 3  # 黄灯持续时长(秒)

# 网络连接配置
CONTROLLER_HOST = "0.0.0.0"
CONTROLLER_PORT = "50051"
AGENT_HOST = "0.0.0.0"
AGENT_PORT = "50052"


# 相位到灯组颜色的映射,根据渠化图，共有39个逻辑灯位
PHASE_TO_LIGHTS = {
    0: "GGGGGGrrrrrrrrrrrrrGGGGGGrrrrrrrrrrrrrr",
    1: "rrrrrrGGGGrrrrrrrrrrrrrrrGGGGrrrrrrrrrr",
    2: "rrrrrrrrrrGGGGGGrrrrrrrrrrrrrGGGGGGrrrr",
    3: "rrrrrrrrrrrrrrrrGGGrrrrrrrrrrrrrrrrGGGG",
    4: "yyyyyyrrrrrrrrrrrrryyyyyyrrrrrrrrrrrrrr",
    5: "rrrrrryyyyrrrrrrrrrrrrrrryyyyrrrrrrrrrr",
    6: "rrrrrrrrrryyyyyyrrrrrrrrrrrrryyyyyyrrrr",
    7: "rrrrrrrrrrrrrrrryyyrrrrrrrrrrrrrrrryyyy"
}

# 定义相位转换规则
PHASE_TRANSITIONS = {
    0: 4,  # 南北直行绿灯 -> 南北直行黄灯
    1: 5,  # 南北左转绿灯 -> 南北左转黄灯
    2: 6,  # 东西直行绿灯 -> 东西直行黄灯
    3: 7,  # 东西左转绿灯 -> 东西左转黄灯
}

# 相位序列定义
PHASE_SEQUENCE = [0, 1, 2, 3]  # 主相位序列:南北直行->南北左转->东西直行->东西左转
