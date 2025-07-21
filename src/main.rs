use financial_data_pipeline::processor::channels::{MarketDataConsumer, MarketDataProducer};
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting up!");
    let (tx, rx) = mpsc::channel(32);

    let symbols = vec!["AAPL", "GOOGL", "MSFT"];

    for symbol in symbols {
        let producer_tx = tx.clone();
        let symbol_owned = symbol.to_string();

        tokio::spawn(async move {
            let producer = MarketDataProducer::new(producer_tx, symbol_owned);
            if let Err(e) = producer.start_producing().await {
                eprintln!("Producer error: {e}");
            }
        });
    }

    drop(tx);

    let mut consumer = MarketDataConsumer::new(rx);
    consumer.start_consuming().await?;

    let _ = sleep(Duration::from_secs(10)).await;
    Ok(())
}
