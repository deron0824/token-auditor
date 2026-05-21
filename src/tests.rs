#[cfg(test)]
mod tests {
    use crate::audit::token::{TokenCalculator, TokenStats};
    use crate::audit::cost::{CostCalculator, CostResult};
    use crate::config::{Config, ModelPrice};

    #[test]
    fn test_token_count() {
        let calculator = TokenCalculator::new().unwrap();
        
        // 测试简单的英文文本
        let text = "Hello, world!";
        let count = calculator.count_tokens(text);
        assert!(count > 0, "Token count should be greater than 0");
    }

    #[test]
    fn test_token_stats() {
        let calculator = TokenCalculator::new().unwrap();
        
        let input = "Hello";
        let output = "Hi there!";
        
        let stats = calculator.calculate_stats(input, output);
        
        assert!(stats.input_tokens > 0);
        assert!(stats.output_tokens > 0);
        assert_eq!(stats.total_tokens, stats.input_tokens + stats.output_tokens);
    }

    #[test]
    fn test_cost_calculation() {
        let calculator = CostCalculator::new(1.2);
        
        let stats = TokenStats {
            input_tokens: 1000,
            output_tokens: 500,
            total_tokens: 1500,
        };
        
        let price = ModelPrice {
            input: 0.0000025,  // $2.50 / 1M tokens
            output: 0.000010,  // $10.00 / 1M tokens
        };
        
        let result = calculator.calculate_cost(&stats, &price);
        
        // 计算预期成本
        let expected_input_cost = 1000.0 * 0.0000025;
        let expected_output_cost = 500.0 * 0.000010;
        let expected_total = expected_input_cost + expected_output_cost;
        
        assert!((result.input_cost - expected_input_cost).abs() < 0.000001);
        assert!((result.output_cost - expected_output_cost).abs() < 0.000001);
        assert!((result.total_cost - expected_total).abs() < 0.000001);
        
        // 溢价阈值应该是 total_cost * 1.2
        let expected_threshold = expected_total * 1.2;
        assert!((result.premium_threshold - expected_threshold).abs() < 0.000001);
        
        // 应该判定为正常（因为没有实际扣费对比，默认正常）
        assert!(result.is_normal);
    }

    #[test]
    fn test_cost_format() {
        assert_eq!(CostCalculator::format_cost(0.00768), "$0.007680");
        assert_eq!(CostCalculator::format_cost(0.10), "$0.100000");
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        
        assert_eq!(config.proxy.listen, "127.0.0.1:11435");
        assert!(!config.audit.enabled);
        assert_eq!(config.audit.tolerance, 1.2);
        assert_eq!(config.display.mode, "terminal");
        assert_eq!(config.display.alert_threshold, 0.10);
        assert_eq!(config.storage.snapshot_interval, 300);
    }
}
