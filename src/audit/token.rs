use anyhow::Result;
use tiktoken_rs::{cl100k_base, p50k_base, p50k_edit, CoreBPE};

/// Token 计算器
pub struct TokenCalculator {
    bpe: CoreBPE,
}

/// Token 统计结果
#[derive(Debug, Clone)]
pub struct TokenStats {
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub total_tokens: usize,
}

impl TokenCalculator {
    /// 创建新的 Token 计算器（使用 OpenAI 默认编码）
    pub fn new() -> Result<Self> {
        let bpe = cl100k_base()?;
        Ok(Self { bpe })
    }

    /// 根据模型名称创建 Token 计算器
    pub fn from_model(model: &str) -> Result<Self> {
        let bpe = Self::get_bpe_for_model(model)?;
        Ok(Self { bpe })
    }

    /// 计算文本的 Token 数量
    pub fn count_tokens(&self, text: &str) -> usize {
        self.bpe.encode_ordinary(text).len()
    }

    /// 计算完整的 Token 统计
    pub fn calculate_stats(&self, input_text: &str, output_text: &str) -> TokenStats {
        let input_tokens = self.count_tokens(input_text);
        let output_tokens = self.count_tokens(output_text);
        let total_tokens = input_tokens + output_tokens;

        TokenStats {
            input_tokens,
            output_tokens,
            total_tokens,
        }
    }

    /// 根据模型名称获取对应的 BPE 编码器
    fn get_bpe_for_model(model: &str) -> Result<CoreBPE> {
        // OpenAI GPT-4o, GPT-3.5-Turbo 等使用 cl100k_base
        if model.contains("gpt-4") || model.contains("gpt-3.5") {
            cl100k_base().map_err(|e| anyhow::anyhow!("加载编码器失败: {}", e))
        }
        // GPT-3 系列使用 p50k_base
        else if model.contains("gpt-3") {
            p50k_base().map_err(|e| anyhow::anyhow!("加载编码器失败: {}", e))
        }
        // Code 模型使用 p50k_edit
        else if model.contains("code") {
            p50k_edit().map_err(|e| anyhow::anyhow!("加载编码器失败: {}", e))
        }
        // 默认使用 cl100k_base
        else {
            cl100k_base().map_err(|e| anyhow::anyhow!("加载编码器失败: {}", e))
        }
    }
}

impl Default for TokenCalculator {
    fn default() -> Self {
        Self::new().expect("Failed to create TokenCalculator")
    }
}
