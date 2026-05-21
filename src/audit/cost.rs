use crate::audit::token::TokenStats;
use crate::config::ModelPrice;

/// 成本计算结果
#[derive(Debug, Clone)]
pub struct CostResult {
    pub input_cost: f64,
    pub output_cost: f64,
    pub total_cost: f64,
    pub premium_threshold: f64,
    pub is_normal: bool,
}

/// 成本计算器
pub struct CostCalculator {
    tolerance: f64,
}

impl CostCalculator {
    /// 创建新的成本计算器
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// 计算成本
    pub fn calculate_cost(&self, stats: &TokenStats, price: &ModelPrice) -> CostResult {
        // 计算输入输出成本
        let input_cost = stats.input_tokens as f64 * price.input;
        let output_cost = stats.output_tokens as f64 * price.output;
        let total_cost = input_cost + output_cost;

        // 计算溢价阈值
        let premium_threshold = total_cost * self.tolerance;

        // 判断是否正常（这里简化处理，实际需要从响应中提取实际扣费）
        // 暂时假设实际扣费等于计算成本
        let is_normal = total_cost < premium_threshold;

        CostResult {
            input_cost,
            output_cost,
            total_cost,
            premium_threshold,
            is_normal,
        }
    }

    /// 格式化成本为美元字符串
    pub fn format_cost(cost: f64) -> String {
        format!("${:.6}", cost)
    }

    /// 格式化带计算过程的成本
    pub fn format_cost_detail(count: usize, price: f64) -> String {
        let cost = count as f64 * price;
        format!("{} ({} × ${:.7})", Self::format_cost(cost), count, price)
    }
}
