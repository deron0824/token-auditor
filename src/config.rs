use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 完整配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub proxy: ProxyConfig,
    #[serde(default)]
    pub audit: AuditConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub storage: StorageConfig,
}

/// 代理配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// 监听地址
    #[serde(default = "default_listen_address")]
    pub listen: String,
}

/// 审计配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// 是否开启成本审计模式
    #[serde(default)]
    pub enabled: bool,

    /// 溢价容忍度（默认 1.2，即 20% 溢价容忍）
    #[serde(default = "default_tolerance")]
    pub tolerance: f64,

    /// 价格配置
    #[serde(default)]
    pub prices: PriceConfig,
}

/// 价格配置（按厂商组织）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriceConfig {
    #[serde(default)]
    pub openai: HashMap<String, ModelPrice>,
    #[serde(default)]
    pub anthropic: HashMap<String, ModelPrice>,
    #[serde(default)]
    pub google: HashMap<String, ModelPrice>,
}

/// 模型价格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPrice {
    pub input: f64,
    pub output: f64,
}

/// 显示配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    /// 显示模式: terminal / vscode / both
    #[serde(default = "default_display_mode")]
    pub mode: String,

    /// 成本告警阈值（美元）
    #[serde(default = "default_alert_threshold")]
    pub alert_threshold: f64,
}

/// 存储配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// JSON 快照路径
    #[serde(default = "default_snapshot_path")]
    pub snapshot_path: String,

    /// 快照间隔（秒）
    #[serde(default = "default_snapshot_interval")]
    pub snapshot_interval: u64,
}

// 默认值函数
fn default_listen_address() -> String {
    "127.0.0.1:11435".to_string()
}

fn default_tolerance() -> f64 {
    1.2
}

fn default_display_mode() -> String {
    "terminal".to_string()
}

fn default_alert_threshold() -> f64 {
    0.10
}

fn default_snapshot_path() -> String {
    "~/.token-auditor/audit-snapshot.json".to_string()
}

fn default_snapshot_interval() -> u64 {
    300
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            listen: default_listen_address(),
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            tolerance: default_tolerance(),
            prices: PriceConfig::default(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            mode: default_display_mode(),
            alert_threshold: default_alert_threshold(),
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            snapshot_path: default_snapshot_path(),
            snapshot_interval: default_snapshot_interval(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxy: ProxyConfig::default(),
            audit: AuditConfig::default(),
            display: DisplayConfig::default(),
            storage: StorageConfig::default(),
        }
    }
}

impl Config {
    /// 从文件加载配置
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// 从默认路径加载配置
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            match Self::from_file(&config_path) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("⚠️  配置文件加载失败: {}", e);
                    eprintln!("   使用默认配置");
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }

    /// 获取配置文件路径
    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".token-auditor")
            .join("config.toml")
    }

    /// 保存配置到文件
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// 根据模型名称获取价格
    pub fn get_model_price(&self, model: &str) -> Option<ModelPrice> {
        // 查找所有厂商的价格
        for prices in [
            &self.audit.prices.openai,
            &self.audit.prices.anthropic,
            &self.audit.prices.google,
        ] {
            if let Some(price) = prices.get(model) {
                return Some(price.clone());
            }
        }
        None
    }
}
