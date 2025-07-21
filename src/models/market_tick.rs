// this is where the async functions will go

use chrono::{DateTime, Utc};
use rand::{Rng, random_range};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    pub symbol: String,
    pub price: Decimal,
    pub volume: u64,
    pub timestamp: DateTime<Utc>,
}

impl MarketTick {
    pub fn new(symbol: String, price: Decimal, volume: u64) -> Self {
        MarketTick {
            symbol,
            price,
            volume,
            timestamp: Utc::now(),
        }
    }

    pub fn is_significant_volume(&self) -> bool {
        self.volume > 1000
    }
}

use tokio::time::{Duration, sleep};

pub async fn fetch_market_data(symbol: &str) -> Result<MarketTick, String> {
    let mut rng = rand::rng();
    // TODO: Simulate network delay with sleep(Duration::from_millis(100))
    // TODO: Return a MarketTick with random price and volume
    // TODO: Return Err for symbol "INVALID"
    let _ = sleep(Duration::from_millis(100));
    if symbol == "INVALID" {
        return Err(format!("Invalid symbol {symbol}"));
    }
    let cents = rng.random_range(100u64..10000u64);
    let price = Decimal::new(cents as i64, 2);
    Ok(MarketTick::new(
        symbol.to_string(),
        price,
        random_range(0..2000),
    ))
}

pub async fn fetch_multiple_symbols(symbols: Vec<&str>) -> Vec<MarketTick> {
    // TODO: fetch data for all symbols concurrently
    //  TODO: Filter out any errors
    let handles: Vec<_> = symbols
        .into_iter()
        .map(|symbol| {
            let owned_symbol = symbol.to_string();
            tokio::spawn(async move { fetch_market_data(&owned_symbol).await })
        })
        .collect();

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(Ok(tick)) = handle.await {
            results.push(tick)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_significant_volume() {
        let sig_tick = MarketTick::new(String::from("AMZN"), Decimal::new(1000, 2), 1001);
        assert!(sig_tick.is_significant_volume());

        let insig_tick = MarketTick::new(String::from("AMZN"), Decimal::new(1000, 2), 999);
        assert!(!insig_tick.is_significant_volume());
    }

    #[tokio::test]
    async fn test_fetch_market_data() {
        let result = fetch_market_data("BTI").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().symbol, "BTI");

        let bad_result = fetch_market_data("INVALID").await;
        assert!(bad_result.is_err());
    }
}
