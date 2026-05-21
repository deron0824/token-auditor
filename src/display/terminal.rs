use crate::audit::cost::CostResult;
use crate::audit::token::TokenStats;
use colored::Colorize;

/// 显示模式
pub enum DisplayMode {
    Monitor, // 实时监测模式
    Audit,   // 成本审计模式
}

/// 终端显示器
pub struct TerminalDisplay;

impl TerminalDisplay {
    /// 创建新的终端显示器
    pub fn new() -> Self {
        Self
    }

    /// 显示实时监测结果
    pub fn display_monitor(
        &self,
        model: &str,
        hub: &str,
        stats: &TokenStats,
        ttft_ms: u64,
        total_ms: u64,
        tpot: f64,
    ) {
        println!();
        println!(
            "{}",
            "╔══════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║  TokenAuditor - API 请求分析             ║".bright_cyan()
        );
        println!(
            "{}",
            "╠══════════════════════════════════════════╣".bright_cyan()
        );
        println!("{}", format!("║  模型: {:<34}║", model).bright_white());
        println!("{}", format!("║  Hub: {:<35}║", hub).bright_white());
        println!(
            "{}",
            "║                                          ║".bright_white()
        );
        println!(
            "{}",
            "║  Token 消耗:                             ║".bright_white()
        );
        println!(
            "{}",
            format!(
                "║    Input:  {:<6} tokens                ║",
                format_number(stats.input_tokens)
            )
            .bright_green()
        );
        println!(
            "{}",
            format!(
                "║    Output: {:<6} tokens                ║",
                format_number(stats.output_tokens)
            )
            .bright_green()
        );
        println!(
            "{}",
            format!(
                "║    Total:  {:<6} tokens                ║",
                format_number(stats.total_tokens)
            )
            .bright_green()
        );
        println!(
            "{}",
            "║                                          ║".bright_white()
        );
        println!(
            "{}",
            "║  性能指标:                               ║".bright_white()
        );
        println!(
            "{}",
            format!("║    TTFT:     {:<4}ms                      ║", ttft_ms).bright_yellow()
        );
        println!(
            "{}",
            format!("║    总耗时:   {:<4}ms                      ║", total_ms).bright_yellow()
        );
        println!(
            "{}",
            format!("║    吐字速率: {:.1} tokens/sec             ║", tpot).bright_yellow()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════╝".bright_cyan()
        );
        println!();
    }

    /// 显示成本审计结果
    pub fn display_audit(
        &self,
        model: &str,
        hub: &str,
        stats: &TokenStats,
        cost: &CostResult,
        ttft_ms: u64,
        total_ms: u64,
        tpot: f64,
    ) {
        println!();
        println!(
            "{}",
            "╔══════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║  TokenAuditor - 成本审计报告             ║".bright_cyan()
        );
        println!(
            "{}",
            "╠══════════════════════════════════════════╣".bright_cyan()
        );
        println!("{}", format!("║  模型: {:<34}║", model).bright_white());
        println!("{}", format!("║  Hub: {:<35}║", hub).bright_white());
        println!(
            "{}",
            "║                                          ║".bright_white()
        );
        println!(
            "{}",
            "║  Token 消耗:                             ║".bright_white()
        );
        println!(
            "{}",
            format!(
                "║    Input:  {:<6} tokens                ║",
                format_number(stats.input_tokens)
            )
            .bright_green()
        );
        println!(
            "{}",
            format!(
                "║    Output: {:<6} tokens                ║",
                format_number(stats.output_tokens)
            )
            .bright_green()
        );
        println!(
            "{}",
            format!(
                "║    Total:  {:<6} tokens                ║",
                format_number(stats.total_tokens)
            )
            .bright_green()
        );
        println!(
            "{}",
            "║                                          ║".bright_white()
        );
        println!(
            "{}",
            "║  成本估算（基于官方基准价）:             ║".bright_white()
        );
        println!(
            "{}",
            format!(
                "║    Input:  ${:<6}                      ║",
                format!("{:.6}", cost.input_cost)
            )
            .bright_green()
        );
        println!(
            "{}",
            format!(
                "║    Output: ${:<6}                      ║",
                format!("{:.6}", cost.output_cost)
            )
            .bright_green()
        );
        println!(
            "{}",
            format!(
                "║    Total:  ${:<6}                      ║",
                format!("{:.6}", cost.total_cost)
            )
            .bright_green()
        );
        println!(
            "{}",
            "║                                          ║".bright_white()
        );
        println!(
            "{}",
            "║  溢价判定:                               ║".bright_white()
        );
        println!(
            "{}",
            format!(
                "║    官方基准价: ${:<5}                  ║",
                format!("{:.6}", cost.total_cost)
            )
            .bright_white()
        );
        println!(
            "{}",
            format!(
                "║    溢价阈值:   ${:<5}                  ║",
                format!("{:.6}", cost.premium_threshold)
            )
            .bright_yellow()
        );

        // 红绿灯状态
        let status = if cost.is_normal {
            "🟢 正常波动".green()
        } else {
            "🔴 严重溢价预警".red().bold()
        };
        println!(
            "{}",
            format!("║    状态:       {:<22}║", status).bright_white()
        );

        println!(
            "{}",
            "║                                          ║".bright_white()
        );
        println!(
            "{}",
            "║  性能指标:                               ║".bright_white()
        );
        println!(
            "{}",
            format!("║    TTFT:     {:<4}ms                      ║", ttft_ms).bright_yellow()
        );
        println!(
            "{}",
            format!("║    总耗时:   {:<4}ms                      ║", total_ms).bright_yellow()
        );
        println!(
            "{}",
            format!("║    吐字速率: {:.1} tokens/sec             ║", tpot).bright_yellow()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════╝".bright_cyan()
        );
        println!();
    }

    /// 显示成本告警
    pub fn display_cost_alert(&self, cost: f64, threshold: f64) {
        println!();
        println!("{}", "⚠️  成本告警！".red().bold());
        println!("{}", format!("   当前成本: ${:.6}", cost).red());
        println!("{}", format!("   告警阈值: ${:.6}", threshold).red());
        println!();
    }
}

impl Default for TerminalDisplay {
    fn default() -> Self {
        Self::new()
    }
}

/// 格式化数字（添加千分位）
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
