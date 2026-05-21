# 贡献指南

感谢你考虑为 TokenAuditor 做出贡献！

## 如何贡献

### 报告 Bug

如果你发现了 Bug，请在 [GitHub Issues](https://github.com/token-auditor/token-auditor/issues) 中提交 Issue，并包含：

1. **简要描述**：一句话说明问题
2. **复现步骤**：详细的操作步骤
3. **期望行为**：你认为应该发生什么
4. **实际行为**：实际发生了什么
5. **环境信息**：操作系统、Rust 版本、TokenAuditor 版本
6. **日志输出**：相关日志（使用 `RUST_LOG=debug` 获取）

### 功能建议

欢迎提出新功能建议！请在 [GitHub Discussions](https://github.com/token-auditor/token-auditor/discussions) 中讨论：

- 功能描述
- 使用场景
- 预期效果
- 是否有类似工具已经实现

### 代码贡献

#### 1. Fork 仓库

```bash
# Fork 后克隆你的 fork
git clone https://github.com/YOUR_USERNAME/token-auditor.git
cd token-auditor

# 添加上游仓库
git remote add upstream https://github.com/token-auditor/token-auditor.git
```

#### 2. 创建分支

```bash
# 保持与 upstream 同步
git checkout main
git pull upstream main

# 创建特性分支
git checkout -b feature/your-feature-name
```

#### 3. 开发

```bash
# 安装依赖
cargo build

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 调试运行
RUST_LOG=debug cargo run -- start
```

#### 4. 提交

```bash
# 提交变更
git add .
git commit -m "feat: 添加某某功能"

# 推送到你的 fork
git push origin feature/your-feature-name
```

#### 5. 创建 Pull Request

- 在 GitHub 上创建 Pull Request
- 填写清晰的 PR 描述
- 等待 CI 检查通过
- 回应 reviewer 的反馈

## 开发规范

### 代码风格

- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 函数和模块需要添加文档注释

### 命名约定

- **模块/文件**：小写 + 下划线（如 `token_calculator`）
- **结构体/枚举**：PascalCase（如 `TokenCalculator`）
- **函数/变量**：snake_case（如 `calculate_cost`）
- **常量**：UPPER_SNAKE_CASE（如 `MAX_TOKENS`）

### 提交信息

遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Type**:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响代码运行）
- `refactor`: 重构（不是新功能也不是修复）
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

**示例**:
```
feat(proxy): 实现 HTTPS CONNECT 代理支持

- 添加 handle_connect 方法
- 实现双向流量转发
- 更新集成测试

Closes #123
```

### 测试要求

- 新功能必须包含单元测试
- 修改现有功能需要更新相关测试
- 所有测试必须通过：`cargo test`
- 集成测试放在 `tests/` 目录

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_function_name

# 运行集成测试
cargo test --test '*'
```

### 文档要求

- 公共 API 必须有文档注释
- 复杂逻辑需要添加注释说明
- 更新 README 中的相关章节
- 示例代码需要测试可运行

**文档注释示例**:
```rust
/// 计算 Token 使用成本
///
/// # 参数
///
/// * `stats` - Token 使用统计
/// * `price` - 价格配置
///
/// # 返回
///
/// 成本计算结果
///
/// # 示例
///
/// ```
/// use token_auditor::audit::cost::CostCalculator;
///
/// let calculator = CostCalculator::new(1.2);
/// let result = calculator.calculate_cost(&stats, &price);
/// ```
pub fn calculate_cost(&self, stats: &TokenStats, price: &ModelPrice) -> CostResult {
    // 实现
}
```

## 审查流程

1. **自动检查**：
   - CI 自动运行测试
   - `cargo fmt` 检查格式
   - `cargo clippy` 检查代码质量

2. **人工审查**：
   - 代码逻辑是否正确
   - 是否有边界情况未处理
   - 是否符合项目架构设计
   - 文档是否完善

3. **合并**：
   - 至少 1 个 maintainer  approve
   - 所有 CI 检查通过
   - 无未解决的讨论

## 常见问题

### Q: 如何添加新模型支持？

A: 在 `src/audit/prices.rs` 中添加模型价格配置，并更新文档。

### Q: 如何测试代理功能？

A: 使用 `test_multi_api.py` 脚本，需要配置相应的 API Key。

### Q: 如何添加新的 CLI 命令？

A: 在 `src/cli.rs` 中添加命令定义，在 `src/main.rs` 中实现逻辑。

### Q: 如何贡献价格数据？

A: 可以直接提交 PR 修改 `config/default.toml` 或文档中的价格表。

## 行为准则

- 尊重他人，友好交流
- 接受建设性批评
- 关注技术问题，不针对个人
- 帮助新手入门

## 感谢

感谢所有贡献者！你们的付出让 TokenAuditor 变得更好。

---

**开始贡献吧！** 🚀
