# IntelliTraffic - 智能交通信号控制系统

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.12+-blue.svg)](https://www.python.org)

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

## 组件版本

| 组件 | 版本 | 语言 | 描述 |
|------|------|------|------|
| signal-controller | v0.1.0 | Rust 2021 | 信号控制器服务，负责执行信号灯控制指令 |
| traffic-detector | v0.1.0 | Rust 2021 | 交通检测器服务，模拟雷视机采集交通数据 |
| smart-agent | v0.1.0 | Python 3.12+ | 智能代理服务，基于DQN的决策引擎 |

### 主要依赖

**Rust 组件:**
- tokio 1.0+ (异步运行时)
- serde 1.0+ (序列化框架)
- serde_json 1.0+ (JSON处理)

**Python 组件:**
- stable-baselines3 ≥2.5.0 (强化学习框架)
- torch ≥2.6.0 (深度学习框架)
- numpy, asyncio (数据处理与异步支持)

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
- **包管理**: Cargo (Rust) + uv (Python)
- **部署**: Systemd服务化部署

## 环境要求

- **Rust**: 1.70+ (推荐使用 rustup 安装)
- **Python**: 3.12+ 
- **操作系统**: Linux, macOS, Windows
- **内存**: 最低 2GB RAM
- **网络**: 支持 TCP 连接的局域网环境

## 安装与部署

### 开发环境

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd IntelliTraffic
   ```

2. **安装 Rust 依赖**
   ```bash
   # 安装 signal-controller
   cd signal-controller
   cargo build --release
   
   # 安装 traffic-detector
   cd ../traffic-detector
   cargo build --release
   ```

3. **安装 Python 依赖**
   ```bash
   cd ../smart-agent
   uv sync  # 或使用 pip install -e .
   ```

### 生产环境

推荐使用 systemd 服务化部署，确保服务的稳定运行和自动重启。

## 配置说明

- `signal-controller/`: 信号机连接配置
- `traffic-detector/config.toml`: 检测器网络配置
- `smart-agent/config.py`: AI模型和网络参数配置

## 应用场景

- 城市交通信号优化
- 智慧交通系统集成
- 交通流量管理
- 自适应信号控制研究

## 贡献指南

欢迎提交 Issue 和 Pull Request！请确保：

1. 代码符合项目的编码规范
2. 添加必要的测试用例
3. 更新相关文档


## 项目愿景

通过AI技术革新传统交通信号控制，构建更智能、更高效的城市交通管理系统，为智慧城市建设贡献力量。


## 许可证

本项目采用 [MIT 许可证](LICENSE)。

```
MIT License

Copyright (c) 2024 IntelliTraffic Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

*IntelliTraffic - Where Intelligence Meets Traffic Control* 🚦🤖
        