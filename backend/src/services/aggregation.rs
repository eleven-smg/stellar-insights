use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAggregation {
    pub pair: String,
    pub average_price: f64,
    pub volume: f64,
    pub slippage: f64,
}

pub struct AggregatorService {}

impl AggregatorService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_slippage(&self, expected_price: f64, actual_price: f64) -> f64 {
        // ✅ Fixed: Implemented actual slippage formula
        if expected_price == 0.0 {
            return 0.0;
        }
        ((actual_price - expected_price) / expected_price).abs()
    }

    pub async fn aggregate_trades(&self, pair: &str, expected_price: f64, actual_price: f64, volume: f64) -> Result<TradeAggregation> {
        let slippage = self.calculate_slippage(expected_price, actual_price);

        Ok(TradeAggregation {
            pair: pair.to_string(),
            average_price: actual_price,
            volume,
            slippage,
        })
    }
}