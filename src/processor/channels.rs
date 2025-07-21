use crate::models::{MarketTick, fetch_market_data};
use std::time::Duration;
use tokio::sync::mpsc;

pub struct MarketDataProducer {
    tx: mpsc::Sender<MarketTick>,
    symbol: String,
}

impl MarketDataProducer {
    pub fn new(tx: mpsc::Sender<MarketTick>, symbol: String) -> Self {
        MarketDataProducer { tx, symbol }
    }

    pub async fn start_producing(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match fetch_market_data(&self.symbol).await {
                Ok(tick) => {
                    if self.tx.send(tick).await.is_err() {
                        println!("Consumser dropped, stopping producer for {}", self.symbol);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching market data for {}: {}", self.symbol, e);
                }
            }
            let _ = tokio::time::sleep(Duration::from_millis(50)).await;
        }
        Ok(())
    }
}

pub struct MarketDataConsumer {
    rx: mpsc::Receiver<MarketTick>,
}

impl MarketDataConsumer {
    pub fn new(rx: mpsc::Receiver<MarketTick>) -> Self {
        MarketDataConsumer { rx }
    }

    pub async fn start_consuming(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.rx.recv().await {
                Some(tick) => {
                    println!("Tick: {}, {}, {}", tick.symbol, tick.price, tick.volume)
                }
                None => {
                    println!("No messages received. Terminating");
                    break;
                }
            }
        }
        Ok(())
    }
}
