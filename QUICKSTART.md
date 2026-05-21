# 🚀 TokenAuditor 快速入门指南

## 第一步：编译项目

```bash
cd "/Users/chuanlong/Desktop/TokenAuditor + HubRador/TokenAuditor"
cargo build --release
```

编译成功后，二进制文件位于：`target/release/token-auditor`

## 第二步：测试运行

### 1. 查看帮助

```bash
./target/release/token-auditor --help
```

你应该看到：
```
本地 AI 成本监控与分析工具

Usage: token-auditor <COMMAND>

Commands:
  start      启动 HTTP 代理
  benchmark  运行基准测试，对比多个 Hub 的性能和成本
  stats      查看历史统计
  export     导出 JSON 快照
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 2. 运行测试

```bash
cargo test
```

应该看到：
```
running 5 tests
test tests::tests::test_cost_calculation ... ok
test tests::tests::test_default_config ... ok
test tests::tests::test_cost_format ... ok
test tests::tests::test_token_count ... ok
test tests::tests::test_token_stats ... ok

test result: ok. 5 passed; 0 failed
```

## 第三步：启动代理

### 方式 1：基础启动（使用默认配置）

```bash
./target/release/token-auditor start
```

输出：
```
✓ TokenAuditor 已启动
📡 监听地址: 127.0.0.1:11435
📝 按 Ctrl+C 停止
```

### 方式 2：自定义端口

```bash
./target/release/token-auditor start --listen 127.0.0.1:8080
```

## 第四步：配置环境变量

在**另一个终端**中运行：

```bash
export HTTP_PROXY=http://127.0.0.1:11435
export HTTPS_PROXY=http://127.0.0.1:11435
```

## 第五步：测试代理（可选）

### Python 测试脚本

创建 `test_proxy.py`：

```python
from openai import OpenAI

# 配置代理（如果已经设置了环境变量，这步可跳过）
# import os
# os.environ['HTTP_PROXY'] = 'http://127.0.0.1:11435'
# os.environ['HTTPS_PROXY'] = 'http://127.0.0.1:11435'

client = OpenAI(
    api_key="your-api-key",  # 替换为你的 API Key
    base_url="https://api.openai.com/v1"
)

response = client.chat.completions.create(
    model="gpt-4o",
    messages=[{"role": "user", "content": "Hello, TokenAuditor!"}]
)

print(response.choices[0].message.content)
```

运行：
```bash
python test_proxy.py
```

### 预期效果

TokenAuditor 终端会显示：

```
╔══════════════════════════════════════════════════════╗
║  欢迎使用 TokenAuditor - 成本审计说明                ║
╠══════════════════════════════════════════════════════╣
║                                                      ║
║  📊 算账公式：                                       ║
║  官方基准价 × 实际消耗Token × 容忍度 = 溢价阈值      ║
║                                                      ║
║  🚦 红绿灯判定：                                     ║
║  🟢 绿色：实际扣费 < 溢价阈值（正常波动）            ║
║  🔴 红色：实际扣费 ≥ 溢价阈值（严重溢价预警）        ║
║                                                      ║
║  ⚙️  默认容忍度：1.2（20% 溢价）                     ║
║  可在 config.toml 中调整 tolerance 值                ║
║                                                      ║
║  ⚠️  免责声明：                                      ║
║  预估成本基于官方基准价推算，仅供参考                ║
║  第三方平台可能有不同计费策略                        ║
╚══════════════════════════════════════════════════════╝

╔══════════════════════════════════════════╗
║  TokenAuditor - API 请求分析             ║
╠══════════════════════════════════════════╣
║  模型: gpt-4o                            ║
║  Hub: api.openai.com                     ║
║                                          ║
║  Token 消耗:                             ║
║    Input:  15 tokens                     ║
║    Output: 8 tokens                      ║
║    Total:  23 tokens                     ║
║                                          ║
║  性能指标:                               ║
║    TTFT:     35ms                        ║
║    总耗时:   350ms                       ║
║    吐字速率: 22.9 tokens/sec             ║
╚══════════════════════════════════════════╝
```

## 第六步：开启成本审计（可选）

### 1. 创建配置文件

```bash
mkdir -p ~/.token-auditor
cp config/default.toml ~/.token-auditor/config.toml
```

### 2. 编辑配置

```bash
nano ~/.token-auditor/config.toml
```

修改：
```toml
[audit]
enabled = true  # 改为 true
tolerance = 1.2

[audit.prices.openai]
"gpt-4o" = { input = 0.0000025, output = 0.000010 }
```

### 3. 重启代理

```bash
# Ctrl+C 停止当前代理
./target/release/token-auditor start
```

现在再次发送请求，会显示成本审计报告！

## 第七步：查看快照

代理运行期间会每 5 分钟自动生成快照：

```bash
cat ~/.token-auditor/audit-snapshot.json
```

内容示例：
```json
{
  "snapshot_at": "2026-05-20T10:30:00Z",
  "total_requests": 15,
  "total_tokens": {
    "input": 15648,
    "output": 7824
  },
  "total_cost_usd": 0.1234,
  "hub_stats": {
    "api.openai.com": {
      "requests": 15,
      "avg_latency_ms": 320,
      "total_cost_usd": 0.1234
    }
  }
}
```

## 常见问题

### Q1: 端口被占用

```
Error: Address already in use
```

解决：更换端口
```bash
./target/release/token-auditor start --listen 127.0.0.1:8080
```

### Q2: 没有看到分析输出

检查：
1. 环境变量是否正确设置
2. API 请求是否真正通过代理（检查请求 URL）
3. 查看日志：`RUST_LOG=debug ./target/release/token-auditor start`

### Q3: 如何停止代理

按 `Ctrl+C`，代理会优雅退出并保存最终快照。

### Q4: 如何查看帮助

```bash
./target/release/token-auditor --help
./target/release/token-auditor start --help
```

## 下一步

- 📖 阅读 [README.md](README.md) 了解完整功能
- 📊 查看 [PROJECT_SUMMARY.md](PROJECT_SUMMARY.md) 了解项目状态
- 🔧 阅读 [DEVELOPMENT.md](DEVELOPMENT.md) 了解开发进度
- 💡 修改 `config/default.toml` 自定义配置

## 需要帮助？

- 查看项目文档
- 提交 Issue
- 参与讨论

---

祝你使用愉快！🎉
