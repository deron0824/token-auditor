use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "token-auditor")]
#[command(about = "本地 AI 成本监控与分析工具", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 启动 HTTP 代理
    Start {
        /// 代理监听地址
        #[arg(short, long, default_value = "127.0.0.1:11435")]
        listen: String,
    },

    /// 运行基准测试，对比多个 Hub 的性能和成本
    Benchmark {
        /// 测试模型名称
        #[arg(short, long)]
        model: String,

        /// Hub 地址列表（逗号分隔）
        #[arg(short, long)]
        hubs: String,

        /// 测试次数
        #[arg(short, long, default_value = "10")]
        iterations: usize,
    },

    /// 查看历史统计
    Stats {
        /// 快照文件路径
        #[arg(short, long)]
        snapshot: Option<String>,
    },

    /// 导出 JSON 快照
    Export {
        /// 输出文件路径
        #[arg(short, long)]
        output: String,
    },
}
