use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

use super::cache::CacheSummary;

/// JSON 快照结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub snapshot_at: String,
    pub total_requests: usize,
    pub total_tokens: TokenSummary,
    pub total_cost_usd: f64,
    pub hub_stats: std::collections::HashMap<String, HubSnapshot>,
}

/// Token 摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSummary {
    pub input: usize,
    pub output: usize,
}

/// Hub 快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSnapshot {
    pub requests: usize,
    pub avg_latency_ms: f64,
    pub total_cost_usd: f64,
}

/// 快照管理器
pub struct SnapshotManager {
    snapshot_path: PathBuf,
    interval_secs: u64,
}

impl SnapshotManager {
    /// 创建新的快照管理器
    pub fn new(snapshot_path: &str, interval_secs: u64) -> Result<Self> {
        let path = expand_path(snapshot_path)?;

        // 确保父目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        info!("快照管理器已初始化: {:?}", path);

        Ok(Self {
            snapshot_path: path,
            interval_secs,
        })
    }

    /// 生成快照
    pub fn generate_snapshot(&self, summary: &CacheSummary) -> Result<()> {
        let snapshot = Snapshot {
            snapshot_at: Utc::now().to_rfc3339(),
            total_requests: summary.total_requests,
            total_tokens: TokenSummary {
                input: summary.total_input_tokens,
                output: summary.total_output_tokens,
            },
            total_cost_usd: summary.total_cost,
            hub_stats: summary
                .hub_stats
                .iter()
                .map(|(hub, stats)| {
                    (
                        hub.clone(),
                        HubSnapshot {
                            requests: stats.requests,
                            avg_latency_ms: if stats.requests > 0 {
                                stats.total_latency as f64 / stats.requests as f64
                            } else {
                                0.0
                            },
                            total_cost_usd: stats.total_cost,
                        },
                    )
                })
                .collect(),
        };

        // 序列化并写入文件
        let json = serde_json::to_string_pretty(&snapshot)?;
        fs::write(&self.snapshot_path, json)?;

        info!(
            "快照已保存: {:?} ({} 请求)",
            self.snapshot_path, summary.total_requests
        );

        Ok(())
    }

    /// 加载快照
    pub fn load_snapshot(&self) -> Result<Option<Snapshot>> {
        if !self.snapshot_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.snapshot_path)?;
        let snapshot: Snapshot = serde_json::from_str(&content)?;

        Ok(Some(snapshot))
    }

    /// 获取快照间隔
    pub fn interval_secs(&self) -> u64 {
        self.interval_secs
    }

    /// 获取快照路径
    pub fn path(&self) -> &Path {
        &self.snapshot_path
    }
}

/// 展开路径（支持 ~ 符号）
fn expand_path(path: &str) -> Result<PathBuf> {
    if let Some(stripped) = path.strip_prefix('~') {
        if let Some(home) = dirs::home_dir() {
            return Ok(home.join(stripped));
        }
    }
    Ok(PathBuf::from(path))
}
