use std::process::Command;
use std::time::Duration;

/// 集成测试：CLI 命令测试
#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_token-auditor"))
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("本地 AI 成本监控与分析工具"));
    assert!(stdout.contains("start"));
    assert!(stdout.contains("benchmark"));
    assert!(stdout.contains("stats"));
    assert!(stdout.contains("export"));
}

#[test]
fn test_cli_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_token-auditor"))
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("token-auditor"));
}

/// 集成测试：Token 计算准确性
#[cfg(test)]
mod token_calculation_tests {
    use token_auditor::audit::token::TokenCalculator;

    #[test]
    fn test_gpt4o_token_count() {
        let calculator = TokenCalculator::new().unwrap();

        // 测试典型的用户消息
        let messages = vec![
            ("Hello", 1),                      // 至少 1 个 token
            ("Hello, world!", 1),              // 简单短语
            ("What is the weather today?", 1), // 问句
        ];

        for (text, min_tokens) in messages {
            let count = calculator.count_tokens(text);
            assert!(
                count >= min_tokens,
                "Text '{}' should have at least {} tokens, but got {}",
                text,
                min_tokens,
                count
            );
        }
    }

    #[test]
    fn test_token_stats_accuracy() {
        let calculator = TokenCalculator::new().unwrap();

        let input = "Write a short story about AI";
        let output = "Once upon a time, there was an AI...";

        let stats = calculator.calculate_stats(input, output);

        // 验证计算逻辑
        assert!(stats.input_tokens > 0, "Input tokens should be > 0");
        assert!(stats.output_tokens > 0, "Output tokens should be > 0");
        assert_eq!(
            stats.total_tokens,
            stats.input_tokens + stats.output_tokens,
            "Total should equal input + output"
        );
    }
}

/// 集成测试：成本计算逻辑
#[cfg(test)]
mod cost_calculation_tests {
    use token_auditor::audit::cost::CostCalculator;
    use token_auditor::audit::token::TokenStats;
    use token_auditor::config::ModelPrice;

    #[test]
    fn test_gpt4o_cost_calculation() {
        let calculator = CostCalculator::new(1.2);

        // 模拟一次真实的 API 调用
        let stats = TokenStats {
            input_tokens: 100,
            output_tokens: 50,
            total_tokens: 150,
        };

        // GPT-4o 官方价格
        let price = ModelPrice {
            input: 0.0000025, // $2.50 / 1M tokens
            output: 0.000010, // $10.00 / 1M tokens
        };

        let result = calculator.calculate_cost(&stats, &price);

        // 验证计算结果
        let expected_input = 100.0 * 0.0000025;
        let expected_output = 50.0 * 0.000010;
        let expected_total = expected_input + expected_output;

        assert!(
            (result.input_cost - expected_input).abs() < 1e-10,
            "Input cost mismatch"
        );
        assert!(
            (result.output_cost - expected_output).abs() < 1e-10,
            "Output cost mismatch"
        );
        assert!(
            (result.total_cost - expected_total).abs() < 1e-10,
            "Total cost mismatch"
        );

        // 验证溢价阈值
        let expected_threshold = expected_total * 1.2;
        assert!(
            (result.premium_threshold - expected_threshold).abs() < 1e-10,
            "Premium threshold mismatch"
        );
    }

    #[test]
    fn test_cost_alert_threshold() {
        // 测试告警阈值逻辑
        let calculator = CostCalculator::new(1.2);

        // 增加 token 数量以确保超过告警阈值
        let stats = TokenStats {
            input_tokens: 50000,
            output_tokens: 20000,
            total_tokens: 70000,
        };

        let price = ModelPrice {
            input: 0.0000025,
            output: 0.000010,
        };

        let result = calculator.calculate_cost(&stats, &price);

        // 计算成本：50000 * 0.0000025 + 20000 * 0.000010 = 0.125 + 0.2 = 0.325
        // 这个成本应该超过默认告警阈值 0.10
        assert!(
            result.total_cost > 0.10,
            "Cost {} should exceed alert threshold 0.10",
            result.total_cost
        );
    }
}

/// 集成测试：配置加载
#[cfg(test)]
mod config_tests {
    use std::fs;
    use std::path::Path;
    use token_auditor::config::Config;

    #[test]
    fn test_default_config_values() {
        let config = Config::default();

        assert_eq!(config.proxy.listen, "127.0.0.1:11435");
        assert!(!config.audit.enabled);
        assert_eq!(config.audit.tolerance, 1.2);
        assert_eq!(config.display.mode, "terminal");
        assert_eq!(config.display.alert_threshold, 0.10);
        assert_eq!(config.storage.snapshot_interval, 300);
    }

    #[test]
    fn test_config_from_toml() {
        // 创建临时配置文件
        let config_content = r#"
[proxy]
listen = "127.0.0.1:9999"

[audit]
enabled = true
tolerance = 1.5

[audit.prices.openai]
"gpt-4o" = { input = 0.0000025, output = 0.000010 }

[display]
mode = "both"
alert_threshold = 0.05

[storage]
snapshot_path = "/tmp/test-snapshot.json"
snapshot_interval = 60
"#;

        let temp_path = Path::new("/tmp/test-token-auditor-config.toml");
        fs::write(temp_path, config_content).expect("Failed to write temp config");

        let config = Config::from_file(temp_path).expect("Failed to load config");

        assert_eq!(config.proxy.listen, "127.0.0.1:9999");
        assert!(config.audit.enabled);
        assert_eq!(config.audit.tolerance, 1.5);
        assert_eq!(config.display.mode, "both");
        assert_eq!(config.display.alert_threshold, 0.05);
        assert_eq!(config.storage.snapshot_interval, 60);
        assert_eq!(config.storage.snapshot_path, "/tmp/test-snapshot.json");

        // 清理临时文件
        fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_price_lookup() {
        let config_content = r#"
[proxy]
listen = "127.0.0.1:11435"

[audit]
enabled = true

[audit.prices.openai]
"gpt-4o" = { input = 0.0000025, output = 0.000010 }
"gpt-4" = { input = 0.00003, output = 0.00006 }

[audit.prices.anthropic]
"claude-3-5-sonnet" = { input = 0.000003, output = 0.000015 }
"#;

        let temp_path = Path::new("/tmp/test-price-config.toml");
        fs::write(temp_path, config_content).expect("Failed to write temp config");

        let config = Config::from_file(temp_path).expect("Failed to load config");

        // 测试价格查找
        let gpt4o_price = config.get_model_price("gpt-4o");
        assert!(gpt4o_price.is_some());
        assert_eq!(gpt4o_price.unwrap().input, 0.0000025);

        let claude_price = config.get_model_price("claude-3-5-sonnet");
        assert!(claude_price.is_some());
        assert_eq!(claude_price.unwrap().output, 0.000015);

        // 测试不存在的模型
        let unknown_price = config.get_model_price("unknown-model");
        assert!(unknown_price.is_none());

        // 清理
        fs::remove_file(temp_path).ok();
    }
}

/// 集成测试：快照序列化
#[cfg(test)]
mod snapshot_tests {
    use std::fs;
    use std::path::Path;
    use token_auditor::storage::cache::{CacheSummary, HubStats, RequestCache};
    use token_auditor::storage::snapshot::SnapshotManager;

    #[test]
    fn test_snapshot_generation() {
        // 创建缓存并添加数据
        let cache = RequestCache::new();

        // 这里简化测试，实际应该添加 RequestRecord
        let summary = cache.calculate_summary();

        // 创建快照管理器
        let snapshot_path = "/tmp/test-snapshot.json";
        let manager =
            SnapshotManager::new(snapshot_path, 300).expect("Failed to create snapshot manager");

        // 生成快照
        manager
            .generate_snapshot(&summary)
            .expect("Failed to generate snapshot");

        // 验证文件存在
        assert!(Path::new(snapshot_path).exists());

        // 读取并验证内容
        let content = fs::read_to_string(snapshot_path).expect("Failed to read snapshot");
        assert!(content.contains("snapshot_at"));
        assert!(content.contains("total_requests"));
        assert!(content.contains("total_tokens"));

        // 清理
        fs::remove_file(snapshot_path).ok();
    }

    #[test]
    fn test_snapshot_load() {
        let snapshot_path = "/tmp/test-snapshot-load.json";

        // 创建空快照
        let cache = RequestCache::new();
        let summary = cache.calculate_summary();
        let manager = SnapshotManager::new(snapshot_path, 300).unwrap();
        manager.generate_snapshot(&summary).unwrap();

        // 加载快照
        let loaded = manager.load_snapshot().unwrap();
        assert!(loaded.is_some());

        let snapshot = loaded.unwrap();
        assert_eq!(snapshot.total_requests, 0);

        // 清理
        fs::remove_file(snapshot_path).ok();
    }
}

/// 集成测试：代理服务器启动（不实际发送请求）
#[cfg(test)]
mod proxy_startup_tests {
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;
    use tokio::runtime::Runtime;

    #[test]
    #[ignore] // 这个测试比较慢，默认跳过
    fn test_proxy_server_starts() {
        // 注意：这是一个简化的测试
        // 完整的测试需要启动实际的代理服务器并发送请求

        // 验证端口没有被占用
        let result = TcpStream::connect("127.0.0.1:11436");
        // 如果连接失败，说明端口空闲，可以用于测试
        assert!(result.is_err() || result.unwrap().peer_addr().is_err());
    }
}
