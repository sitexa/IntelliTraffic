# IntelliTraffic - 智能交通信号控制系统

## 项目概述

IntelliTraffic是一个基于强化学习的现代化自适应交通信号控制系统(Adaptive Traffic Signal Control System)，以智能体为核心，实现交通信号的智能化决策和优化控制。

### 核心特性

- 🧠 **AI驱动**: 基于深度强化学习(DQN)的智能决策引擎
- 🚦 **自适应控制**: 根据实时交通流量动态调整信号配时
- 🏗️ **微服务架构**: 分布式设计，支持独立部署和扩展
- ⚡ **高性能**: Rust + Python混合技术栈，兼顾性能与AI能力
- 🔧 **生产就绪**: 完整的配置管理和服务化部署方案

## 系统架构

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Traffic        │    │  Smart          │    │  Signal         │
│  Detector       │───▶│  Agent          │───▶│  Controller     │
│  (雷视机)        │    │  (智能体)        │    │  (信号机)        │
│  Port: 50053    │    │  Port: 50052    │    │  Port: 50051    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
     感知层                  决策层                  执行层
```

## 项目结构

```
IntelliTraffic/
├── signal-controller/     # 信号控制器服务 (Rust)
├── traffic-detector/      # 交通检测器服务 (Rust)
├── smart-agent/          # 智能代理服务 (Python)
└── README.md            # 项目说明文档
```

## 快速开始

### 1. 启动信号控制器
```bash
cd signal-controller
cargo run
```

### 2. 启动交通检测器
```bash
cd traffic-detector
cargo run
```

### 3. 启动智能代理
```bash
cd smart-agent
python main.py
```

## 技术栈

- **后端服务**: Rust (Tokio异步运行时)
- **AI引擎**: Python + PyTorch + Stable-Baselines3
- **通信协议**: TCP Socket
- **配置管理**: TOML配置文件
- **部署**: Systemd服务化部署

## 应用场景

- 城市交通信号优化
- 智慧交通系统集成
- 交通流量管理
- 自适应信号控制研究

## 项目愿景

通过AI技术革新传统交通信号控制，构建更智能、更高效的城市交通管理系统，为智慧城市建设贡献力量。

---

*IntelliTraffic - Where Intelligence Meets Traffic Control* 🚦🤖
        