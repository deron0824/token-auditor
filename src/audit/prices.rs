use crate::config::{Config, ModelPrice};
use std::collections::HashMap;
use tracing::info;

/// 价格管理器
pub struct PriceManager {
    prices: HashMap<String, ModelPrice>,
}

impl PriceManager {
    /// 从配置创建价格管理器
    pub fn from_config(config: &Config) -> Self {
        let mut prices = HashMap::new();

        // 合并所有厂商的价格
        Self::merge_prices(&mut prices, &config.audit.prices.openai);
        Self::merge_prices(&mut prices, &config.audit.prices.anthropic);
        Self::merge_prices(&mut prices, &config.audit.prices.google);

        info!("加载了 {} 个模型的价格配置", prices.len());

        Self { prices }
    }

    /// 获取模型价格
    pub fn get_price(&self, model: &str) -> Option<&ModelPrice> {
        self.prices.get(model)
    }

    /// 检查是否已启用成本审计
    pub fn is_audit_enabled(&self) -> bool {
        !self.prices.is_empty()
    }

    /// 合并价格配置
    fn merge_prices(
        target: &mut HashMap<String, ModelPrice>,
        source: &HashMap<String, ModelPrice>,
    ) {
        for (model, price) in source {
            target.insert(model.clone(), price.clone());
        }
    }

    /// TODO: 动态获取价格（从 API 或 GitHub）
    /// 这是一个占位函数，后续可以实现从在线源获取最新价格
    #[allow(dead_code)]
    pub async fn fetch_latest_prices(&mut self) -> anyhow::Result<()> {
        // 未来实现：
        // 1. 从 OpenAI API 获取价格
        // 2. 从 GitHub Raw 文件获取价格清单
        // 3. 更新本地缓存
        info!("动态价格获取功能暂未实现");
        Ok(())
    }
}
