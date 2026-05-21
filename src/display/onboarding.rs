use colored::Colorize;
use std::sync::atomic::{AtomicBool, Ordering};

/// 首次启动标志（确保只显示一次）
static FIRST_RUN: AtomicBool = AtomicBool::new(true);

/// 用户引导显示器
pub struct OnboardingDisplay;

impl OnboardingDisplay {
    /// 创建新的引导显示器
    pub fn new() -> Self {
        Self
    }

    /// 显示首次启动说明（仅显示一次）
    pub fn show_once(&self) {
        if FIRST_RUN.swap(false, Ordering::SeqCst) {
            self.display_welcome();
        }
    }

    /// 显示欢迎说明
    fn display_welcome(&self) {
        println!();
        println!(
            "{}",
            "╔══════════════════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║  欢迎使用 TokenAuditor - 成本审计说明                ║".bright_cyan()
        );
        println!(
            "{}",
            "╠══════════════════════════════════════════════════════╣".bright_cyan()
        );
        println!(
            "{}",
            "║                                                      ║".bright_white()
        );
        println!(
            "{}",
            "║  📊 算账公式：                                       ║".bright_yellow()
        );
        println!(
            "{}",
            "║  官方基准价 × 实际消耗Token × 容忍度 = 溢价阈值      ║".bright_white()
        );
        println!(
            "{}",
            "║                                                      ║".bright_white()
        );
        println!(
            "{}",
            "║  🚦 红绿灯判定：                                     ║".bright_yellow()
        );
        println!(
            "{}",
            "║  🟢 绿色：实际扣费 < 溢价阈值（正常波动）            ║".bright_green()
        );
        println!(
            "{}",
            "║  🔴 红色：实际扣费 ≥ 溢价阈值（严重溢价预警）        ║".bright_red()
        );
        println!(
            "{}",
            "║                                                      ║".bright_white()
        );
        println!(
            "{}",
            "║  ⚙️  默认容忍度：1.2（20% 溢价）                     ║".bright_yellow()
        );
        println!(
            "{}",
            "║  可在 config.toml 中调整 tolerance 值                ║".bright_white()
        );
        println!(
            "{}",
            "║                                                      ║".bright_white()
        );
        println!(
            "{}",
            "║  ⚠️  免责声明：                                      ║".bright_yellow()
        );
        println!(
            "{}",
            "║  预估成本基于官方基准价推算，仅供参考                ║".bright_white()
        );
        println!(
            "{}",
            "║  第三方平台可能有不同计费策略                        ║".bright_white()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════════════════════╝".bright_cyan()
        );
        println!();
    }
}

impl Default for OnboardingDisplay {
    fn default() -> Self {
        Self::new()
    }
}
