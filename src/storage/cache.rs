use crate::audit::token::TokenStats;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// 请求记录
#[derive(Debug, Clone)]
pub struct RequestRecord {
    pub timestamp: SystemTime,
    pub model: String,
    pub hub: String,
    pub stats: TokenStats,
    pub cost_usd: f64,
    pub latency_ms: u64,
    pub ttft_ms: u64,
    pub tpot: f64,
    pub status_code: u16,
}

/// 内存缓存
#[derive(Clone)]
pub struct RequestCache {
    records: Arc<Mutex<Vec<RequestRecord>>>,
}

impl RequestCache {
    /// 创建新的内存缓存
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 添加请求记录
    pub fn add_record(&self, record: RequestRecord) {
        let mut records = self.records.lock().unwrap();
        records.push(record);
    }

    /// 获取所有记录
    pub fn get_records(&self) -> Vec<RequestRecord> {
        let records = self.records.lock().unwrap();
        records.clone()
    }

    /// 获取记录数量
    pub fn len(&self) -> usize {
        let records = self.records.lock().unwrap();
        records.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        let records = self.records.lock().unwrap();
        records.is_empty()
    }

    /// 清空缓存
    pub fn clear(&self) {
        let mut records = self.records.lock().unwrap();
        records.clear();
    }

    /// 计算累计统计
    pub fn calculate_summary(&self) -> CacheSummary {
        let records = self.records.lock().unwrap();

        let total_requests = records.len();
        let mut total_input_tokens = 0;
        let mut total_output_tokens = 0;
        let mut total_cost = 0.0;
        let mut total_latency = 0;
        let mut hub_stats: std::collections::HashMap<String, HubStats> =
            std::collections::HashMap::new();

        for record in records.iter() {
            total_input_tokens += record.stats.input_tokens;
            total_output_tokens += record.stats.output_tokens;
            total_cost += record.cost_usd;
            total_latency += record.latency_ms;

            // 按 Hub 统计
            let hub_stat = hub_stats.entry(record.hub.clone()).or_insert(HubStats {
                requests: 0,
                total_latency: 0,
                total_cost: 0.0,
            });
            hub_stat.requests += 1;
            hub_stat.total_latency += record.latency_ms;
            hub_stat.total_cost += record.cost_usd;
        }

        let avg_latency = if total_requests > 0 {
            total_latency as f64 / total_requests as f64
        } else {
            0.0
        };

        CacheSummary {
            total_requests,
            total_input_tokens,
            total_output_tokens,
            total_cost,
            avg_latency,
            hub_stats,
        }
    }
}

impl Default for RequestCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Hub 统计
#[derive(Debug, Clone)]
pub struct HubStats {
    pub requests: usize,
    pub total_latency: u64,
    pub total_cost: f64,
}

/// 缓存摘要
#[derive(Debug, Clone)]
pub struct CacheSummary {
    pub total_requests: usize,
    pub total_input_tokens: usize,
    pub total_output_tokens: usize,
    pub total_cost: f64,
    pub avg_latency: f64,
    pub hub_stats: std::collections::HashMap<String, HubStats>,
}
