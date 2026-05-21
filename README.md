# TokenAuditor

**本地 AI 成本监控与分析工具**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/token-auditor/token-auditor/releases)
[![Status](https://img.shields.io/badge/status-v0.1.0%20Alpha-green.svg)](https://github.com/token-auditor/token-auditor/releases)

---

## 📖 简介

TokenAuditor 是一个**本地化**的 AI API 成本监控工具，通过 HTTP 代理透明地拦截和分析大模型 API 请求，帮助你清楚地了解 Token 消耗、成本支出和性能表现。

**核心理念**: 让开发者清楚地知道每一分钱花在了哪里。

### ✨ 核心特性

-  **隐私优先**: 所有数据在本地计算和存储，绝不上传云端
-  **极简使用**: 单文件二进制，配置环境变量即可使用
- 📊 **实时监控**: 拦截 API 请求，实时显示 Token 消耗和性能指标
- 🎯 **双模式架构**: 实时监测（默认）+ 成本审计（可选）
- ⚡ **性能分析**: TTFT、总耗时、吐字速率等多维度指标
-  **多平台支持**: 支持 OpenAI、阿里百炼、火山引擎等多个 AI 平台
-  **零侵入**: 无需修改代码，通过代理透明接入
- 🎨 **可视化反馈**: 终端清晰展示，颜色编码直观易懂

---

## 🚀 快速开始

### 安装

#### 方式 1: 从源码编译（推荐）

```bash
# 克隆仓库
git clone https://github.com/token-auditor/token-auditor.git
cd token-auditor

# 编译
cargo build --release

# 添加到 PATH（可选）
sudo cp target/release/token-auditor /usr/local/bin/
```

#### 方式 2: 使用 cargo install（即将支持）

```bash
cargo install token-auditor
```

### 使用

#### 步骤 1: 启动代理

```bash
./target/release/token-auditor start
```

输出：
```
✓ TokenAuditor 已启动
📡 监听地址: 127.0.0.1:11435
📝 按 Ctrl+C 停止
```

#### 步骤 2: 配置环境变量

```bash
export HTTP_PROXY=http://127.0.0.1:11435
export HTTPS_PROXY=http://127.0.0.1:11435
```

#### 步骤 3: 正常使用 AI API

**Python 示例**:
```python
from openai import OpenAI

# 自动通过代理，无需额外配置
client = OpenAI(api_key="your-api-key")

response = client.chat.completions.create(
    model="gpt-4o",  # 或 qwen-turbo, doubao-lite-32k 等
    messages=[{"role": "user", "content": "Hello!"}]
)

print(response.choices[0].message.content)
```

**Node.js 示例**:
```javascript
import OpenAI from "openai";

// 自动通过代理
const openai = new OpenAI({ apiKey: "your-api-key" });

const response = await openai.chat.completions.create({
  model: "gpt-4o",
  messages: [{ role: "user", content: "Hello!" }],
});

console.log(response.choices[0].message.content);
```

#### 步骤 4: 查看实时反馈

TokenAuditor 会自动拦截并分析 API 调用，在终端显示：

```
╔══════════════════════════════════════════╗
║  TokenAuditor - API 请求分析             ║
╠══════════════════════════════════════════╣
║  模型: gpt-4o                            
║  Hub: api.openai.com                     
║                                          ║
║  Token 消耗:                             ║
║    Input:  25 tokens                     ║
║    Output: 10 tokens                     ║
║    Total:  35 tokens                     ║
║                                          ║
║  性能指标:                               ║
║    TTFT:     50ms                        ║
║    总耗时:   350ms                       ║
║    吐字速率: 28.6 tokens/sec             ║
╚══════════════════════════════════════════╝
```

---

## 📋 功能特性

### ✅ 已实现

- **HTTP/HTTPS 代理**: 透明拦截 OpenAI/Anthropic/阿里百炼/火山引擎等平台 API 请求
- **Token 计算**: 集成 tiktoken-rs，支持 GPT-4/Claude/Qwen/Doubao 等模型
- **双模式架构**:
  - 🟢 **实时监测模式（默认）**: 显示 Token 消耗和性能指标（零误差、零争议）
  - 📊 **成本审计模式（可选）**: 计算成本、溢价判定、红绿灯预警
- **终端可视化**: 彩色输出，清晰的 ASCII 框展示
- **用户引导**: 首次启动展示算账公式和免责声明
- **数据存储**: 内存缓存 + JSON 快照持久化
- **CLI 命令**: start/benchmark/stats/export
- **配置管理**: TOML 格式，支持默认值和自定义
- **多平台测试**: 提供测试脚本支持 OpenAI/阿里百炼/火山引擎

### 🚧 开发中

- [ ] 完整的基准测试实现
- [ ] VS Code 插件集成
- [ ] 动态价格获取（从官方 API）
- [ ] 更多模型支持（Claude 3.5 Sonnet 等）
- [ ] 性能趋势图表
- [ ] 社区基准数据共享

---

## 🎯 双模式架构

TokenAuditor 采用**实时监测（默认）+ 成本审计（可选）**的双模式设计，规避第三方平台计费策略不透明带来的准确性风险。

### 模式一：实时监测模式（默认）

**触发条件**: 未配置价格库或主动关闭计费审计

**功能表现**:
- 仅作为客观的流量监测器
- 显示 Token 消耗量（Prompt/Completion/Total）
- 显示性能指标（TTFT、总耗时、TPOT）
- **零误差、零争议**

### 模式二：成本审计模式（可选）

**触发条件**: 在配置文件中主动开启计费审计，并填入官方基准价格

**功能表现**:
- 激活"算账"逻辑与"红绿灯"视觉警示
- 基于官方基准价进行溢价偏差计算
- 输出预估金额和溢价判定

**算账公式**:
```
溢价阈值 = 官方基准价 × 实际消耗Token × tolerance

🟢 绿色状态：实际扣费 < 溢价阈值（正常波动）
 红色状态：实际扣费 ≥ 溢价阈值（严重溢价预警）
```

---

##  多平台支持

TokenAuditor 已通过测试支持以下平台：

| 平台 | 模型示例 | API 地址 |
|------|---------|----------|
| OpenAI | gpt-4o, gpt-3.5-turbo | api.openai.com |
| 阿里百炼 | qwen-turbo, qwen-plus | dashscope.aliyuncs.com |
| 火山引擎 | doubao-lite-32k, doubao-pro-32k | ark.cn-beijing.volces.com |

**测试脚本**: 项目提供了 `test_multi_api.py` 用于快速测试多平台 API。

```bash
# 自动检测并测试所有已配置的 API Key
python test_multi_api.py
```

---

## ⚙️ 配置

### 配置文件

创建 `~/.token-auditor/config.toml`:

```toml
[proxy]
# 代理监听地址
listen = "127.0.0.1:11435"

# ============================================
# 成本审计模式配置（可选）
# ============================================
# 默认情况下，TokenAuditor 仅显示 Token 消耗和性能指标
# 如需开启成本审计，请设置 enabled = true 并填写官方基准价格

[audit]
# 是否开启成本审计模式（默认 false）
enabled = false

# 溢价容忍度（默认 1.2，即 20% 溢价容忍）
tolerance = 1.2

# 官方基准价格（仅填写大模型厂商的官方价格）
[audit.prices.openai]
"gpt-4o" = { input = 0.0000025, output = 0.000010 }

[audit.prices.dashscope]
"qwen-turbo" = { input = 0.0000008, output = 0.0000020 }

[audit.prices.volcengine]
"doubao-lite-32k" = { input = 0.0000008, output = 0.0000020 }

[display]
# 显示模式: terminal / vscode / both
mode = "terminal"

# 成本告警阈值（美元）
alert_threshold = 0.10

[storage]
# JSON 快照路径
snapshot_path = "~/.token-auditor/audit-snapshot.json"

# 快照间隔（秒）
snapshot_interval = 300
```

---

## 🛠️ CLI 命令

```bash
# 启动代理
token-auditor start

# 使用自定义端口
token-auditor start --listen 127.0.0.1:8080

# 运行基准测试
token-auditor benchmark --model gpt-4o --hubs openai,hub-a,hub-b

# 查看历史统计
token-auditor stats

# 导出 JSON 快照
token-auditor export --output report.json

# 查看版本
token-auditor --version

# 查看帮助
token-auditor --help
```

---

## 🧪 测试

```bash
# 运行所有测试（单元测试 + 集成测试）
cargo test

# 运行单元测试
cargo test --lib

# 运行集成测试
cargo test --test '*'

# 运行特定测试
cargo test test_gpt4o_cost_calculation

# 显示测试输出
cargo test -- --nocapture
```

**测试覆盖**:
- ✅ 16 个测试用例（5 个单元 + 11 个集成）
- ✅ Token 计算准确性验证
- ✅ 成本计算精度测试
- ✅ 配置加载测试
- ✅ 快照持久化测试
- ✅ 多平台 API 集成测试

---

## 🛠️ 开发指南

### 环境要求

- Rust 1.70+
- Cargo
- Python 3.8+ （用于测试脚本）

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/token-auditor/token-auditor.git
cd token-auditor

# 安装依赖
cargo build

# 调试模式运行
cargo run -- start

# 查看详细日志
RUST_LOG=debug cargo run -- start

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 运行测试
cargo test
```

### 编译发布版本

```bash
# macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

---

## 📊 示例输出

### 实时监测模式

```
╔══════════════════════════════════════════╗
║  TokenAuditor - API 请求分析             ║
╠══════════════════════════════════════════╣
║  模型: gpt-4o                            ║
║  Hub: api.openai.com                     ║
║                                          ║
║  Token 消耗:                             ║
║    Input:  1,024 tokens                  ║
║    Output: 512 tokens                    ║
║    Total:  1,536 tokens                  ║
║                                          ║
║  性能指标:                               ║
║    TTFT:     320ms                       ║
║    总耗时:   1,250ms                     ║
║    吐字速率: 45.2 tokens/sec             ║
╚══════════════════════════════════════════╝
```

### 成本审计模式

```
╔══════════════════════════════════════════╗
║  TokenAuditor - 成本审计报告             ║
╠══════════════════════════════════════════╣
║  模型: gpt-4o                            ║
║  Token 消耗:                             ║
║    Input:  1,024 tokens                  ║
║    Output: 512 tokens                    ║
║    Total:  1,536 tokens                  ║
║                                          ║
║  成本估算（基于官方基准价）:             ║
║    Input:  $0.002560                     ║
║    Output: $0.005120                     ║
║    Total:  $0.007680                     ║
║                                          ║
║  溢价判定:                               ║
║    官方基准价: $0.007680                 ║
║    溢价阈值:   $0.009216                 ║
║    状态:       🟢 正常波动               ║
║                                          ║
║  性能指标:                               ║
║    TTFT:     320ms                       ║
║    总耗时:   1,250ms                     ║
║    吐字速率: 45.2 tokens/sec             ║
╚══════════════════════════════════════════╝
```

---

## 📁 项目结构

```
TokenAuditor/
├── Cargo.toml              # 项目配置
├── LICENSE                 # MIT 许可证
├── README.md               # 项目说明
├── config/
│   └── default.toml        # 配置模板
├── src/
│   ├── main.rs             # CLI 入口
│   ├── lib.rs              # 公共模块导出
│   ├── cli.rs              # CLI 参数解析
│   ├── config.rs           # 配置管理
│   ├── proxy/              # HTTP/HTTPS 代理
│   │   ├── server.rs       # 代理服务器
│   │   ├── handler.rs      # 请求处理
│   │   └── forwarder.rs    # 请求转发
│   ├── audit/              # 审计计算
│   │   ├── token.rs        # Token 计算
│   │   ├── cost.rs         # 成本计算
│   │   └── prices.rs       # 价格管理
│   ├── display/            # 可视化输出
│   │   ├── terminal.rs     # 终端显示
│   │   └── onboarding.rs   # 用户引导
│   ├── storage/            # 数据存储
│   │   ├── cache.rs        # 内存缓存
│   │   └── snapshot.rs     # JSON 快照
│   ├── runtime/            # 双 Runtime 架构
│   ── tests.rs            # 单元测试
├── tests/                  # 集成测试
│   └── integration_tests.rs
├── benchmarks/             # 性能测试
└── test_multi_api.py       # 多平台 API 测试脚本
```

---

## 🔒 隐私承诺

TokenAuditor 的设计原则是**隐私优先**:

- ✅ 所有数据在本地处理
- ✅ V1.0 无任何云端通信
- ✅ 绝不记录 prompt 或 completion
- ✅ 绝不自动修改用户的 API 配置
- ✅ 所有源代码开源，接受社区监督

---

## ️ 路线图

### V1.0 (当前) - 聚焦"透明"

- [x] HTTP/HTTPS 代理拦截
- [x] Token 计算与成本分析
- [x] 双模式架构（实时监测 + 成本审计）
- [x] 可视化反馈
- [x] JSON 快照持久化
- [x] 多平台支持（OpenAI/阿里百炼/火山引擎）
- [x] 集成测试覆盖

### V1.5 - 聚焦"智能"

- [ ] VS Code 插件
- [ ] 缓存命中率分析
- [ ] Tokenizer 动态校准
- [ ] 性能趋势图表
- [ ] 更多 Hub 适配器
- [ ] 动态价格获取

### V2.0 - 聚焦"生态"

- [ ] 云端价格榜单同步（用户授权）
- [ ] 社区基准数据共享
- [ ] 企业私有化部署支持

---

## 🤝 贡献指南

欢迎贡献！你可以:

1. **提交 Bug**: [GitHub Issues](https://github.com/token-auditor/token-auditor/issues)
2. **功能建议**: [GitHub Discussions](https://github.com/token-auditor/token-auditor/discussions)
3. **更新价格**: 提交 PR 修改价格库
4. **代码贡献**: Fork 仓库，提交 Pull Request

### 开发规范

- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 提交前运行 `cargo fmt` 和 `cargo clippy`
- 新增功能需包含单元测试
- 提交前确保所有测试通过：`cargo test`

---

## 📄 许可证

[MIT License](LICENSE)

---

## ⚠️ 免责声明

- 本项目仅作为开发者的辅助审计工具
- 不参与任何 API Key 的倒卖与资金结算
- 由于各大模型厂商 Tokenizer 算法可能存在黑盒微调，本地计算值与官方值允许存在微小误差（±2%）
- 价格信息仅供参考，请以官方价格页为准

---

## 📮 联系方式

- **GitHub**: [token-auditor](https://github.com/token-auditor/token-auditor)
- **Issues**: [报告问题](https://github.com/token-auditor/token-auditor/issues)
- **Discussions**: [社区讨论](https://github.com/token-auditor/token-auditor/discussions)

---

**⭐ 如果这个项目对你有帮助，请给个 Star！**
