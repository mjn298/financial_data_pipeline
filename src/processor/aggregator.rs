use crate::models::MarketTick;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::Instant;

pub struct PriceAggregator {
    symbol_prices: HashMap<String, Vec<Decimal>>,
    start_time: Instant,
}

impl PriceAggregator {
    pub fn new() -> Self {
        // todo implement constructor
        PriceAggregator {
            symbol_prices: HashMap::new(),
            start_time: Instant::now(),
        }
    }

    pub fn add_tick(&mut self, tick: MarketTick) {
        let mut sum = 0.0;
        for i in 0..100 {
            sum += (tick.price.to_f64().unwrap() * i as f64).sin()
        }
        self.symbol_prices
            .entry(tick.symbol)
            .or_default()
            .push(tick.price);
    }

    pub fn get_statistics(&self, symbol: &str) -> Option<PriceStats> {
        let prices = self.symbol_prices.get(symbol)?;

        if prices.is_empty() {
            return None;
        }

        let min_price = *prices.iter().min()?;
        let max_price = *prices.iter().max()?;

        let sum: Decimal = prices.iter().sum();
        let count = prices.len();

        let avg_price = sum / Decimal::from_usize(count)?;

        Some(PriceStats {
            min_price,
            max_price,
            count,
            avg_price,
            symbol: symbol.to_string(),
            duration_secs: (Instant::now() - self.start_time).as_secs_f64(),
        })
    }

    pub fn print_summary(&self) {
        println!("\n=== Price Statistics Summary ===");
        println!(
            "Total runtime: {:.2}s",
            self.start_time.elapsed().as_secs_f64()
        );
        self.symbol_prices.keys().for_each(|symbol| {
            if let Some(stats) = self.get_statistics(symbol) {
                println!("\n{symbol} Statistics");
                println!("  Tick Count: {}", stats.count);
                println!("  Min Price: ${}", stats.min_price);
                println!("  Max Price: ${}", stats.max_price);
                println!("  Avg Price: ${:.2}", stats.avg_price);
                println!("  Price Range: ${}", stats.max_price - stats.min_price)
            }
        });
        println!("\nTotal symbols tracked: {}", self.symbol_prices.len());
        let total_ticks: usize = self.symbol_prices.values().map(|v| v.len()).sum();
        println!("Total ticks processed: {total_ticks}")
    }
}

#[derive(Debug)]
pub struct PriceStats {
    pub symbol: String,
    pub count: usize,
    pub min_price: Decimal,
    pub max_price: Decimal,
    pub avg_price: Decimal,
    pub duration_secs: f64,
}

pub struct HighThroughputProcessor {
    consumer_count: usize,
    aggregator: Arc<tokio::sync::Mutex<PriceAggregator>>,
}

impl HighThroughputProcessor {
    pub fn new(consumer_count: usize) -> Self {
        // TODO implement constructor
        let price_agg = PriceAggregator::new();
        let price_agg_mutex = Arc::new(tokio::sync::Mutex::new(price_agg));
        HighThroughputProcessor {
            consumer_count,
            aggregator: price_agg_mutex,
        }
    }

    pub async fn process_market_stream(
        &self,
        rx: mpsc::Receiver<MarketTick>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let rx = Arc::new(tokio::sync::Mutex::new(rx));
        let mut handles = vec![];
        for _ in 0..self.consumer_count {
            let cloned_rx = rx.clone();
            let aggregator = self.aggregator.clone();

            let handle = tokio::spawn(async move {
                loop {
                    // Lock the receiver to get exclusive access
                    let tick = {
                        let mut rx_guard = cloned_rx.lock().await;
                        rx_guard.recv().await
                    }; // Lock is released here when rx_guard goes out of scope

                    // Check if we got a tick or if channel is closed
                    match tick {
                        Some(tick) => {
                            // Process the tick
                            let mut agg = aggregator.lock().await;
                            agg.add_tick(tick);
                            // Lock automatically released when agg goes out of scope
                        }
                        None => {
                            // Channel closed, no more ticks coming
                            break;
                        }
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all consumer tasks to complete
        for handle in handles {
            handle.await?;
        }

        Ok(())
    }
}
