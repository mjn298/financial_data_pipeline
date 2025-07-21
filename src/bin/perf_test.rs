use financial_data_pipeline::models::MarketTick;
use financial_data_pipeline::processor::aggregator::{HighThroughputProcessor, PriceAggregator};
use rust_decimal::prelude::*;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::time::Duration;

/// Generates high-frequency market data from multiple producers
async fn generate_high_frequency_data(
    tx: mpsc::Sender<MarketTick>,
    producer_count: usize,
    ticks_per_producer: usize,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Create a vector to store task handles
    let mut handles = vec![];
    let base_price = Decimal::new(100, 2);
    // TODO: Loop to create `producer_count` producers
    // Each producer should:
    // - Have a unique symbol (e.g., "SYM0", "SYM1", etc.)
    // - Generate `ticks_per_producer` MarketTicks
    // - Send them through the channel
    // - Use slightly varying prices to simulate real market data
    for i in 0..producer_count {
        let tx_clone = tx.clone();
        let join_handle = tokio::spawn(async move {
            let symbol = format!("SYM{i}");
            for _ in 0..ticks_per_producer {
                let cents = rand::random_range(-10000i64..10000i64);
                let volume = rand::random_range(0..2000);
                let diff = Decimal::new(cents as i64, 2);
                let new_value = base_price - diff;
                let tick = MarketTick::new(symbol.clone(), new_value, volume);
                tx_clone.send(tick).await.unwrap();
            }
        });
        handles.push(join_handle);
    }
    // TODO: Wait for all producer tasks to complete
    for handle in handles {
        handle.await?;
    }
    Ok(())
}

/// Tests performance with a single consumer
async fn test_single_consumer(
    producer_count: usize,
    ticks_per_producer: usize,
) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Create a channel with appropriate buffer size
    let (tx, mut rx) = mpsc::channel::<MarketTick>(1000);

    // TODO: Record start time
    let start_time = Instant::now();

    // TODO: Spawn the data generation task
    let data_generation_task = tokio::spawn(async move {
        generate_high_frequency_data(tx, producer_count, ticks_per_producer).await
    });
    // TODO: Create a PriceAggregator and process all ticks from the channel
    // Keep track of how many ticks were processed
    let mut price_agg = PriceAggregator::new();
    let mut count = 0;
    while let Some(tick) = rx.recv().await {
        price_agg.add_tick(tick);
        count += 1;
    }

    data_generation_task.await??;

    // TODO: Calculate elapsed time
    let elapsed = start_time.elapsed();
    // TODO: Print results including:
    // - Total ticks processed
    // - Time taken
    // - Throughput (ticks/second)
    // - Call aggregator.print_summary()
    let throughput = f64::from(count) / elapsed.as_secs_f64();
    println!("\n=== Single Consumer Results ===");
    println!("total ticks: {count}");
    println!("throughput: {throughput}");
    println!("elapsed time: {elapsed:?}");
    price_agg.print_summary();
    Ok(elapsed)
}

/// Tests performance with multiple consumers
async fn test_multiple_consumers(
    producer_count: usize,
    ticks_per_producer: usize,
    consumer_count: usize,
) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
    // TODO: Create a channel with appropriate buffer size
    let (tx, rx) = mpsc::channel::<MarketTick>(1000);

    // TODO: Record start time
    let start_time = Instant::now();
    // TODO: Spawn the data generation task
    let data_generation_task = tokio::spawn(async move {
        generate_high_frequency_data(tx, producer_count, ticks_per_producer).await
    });
    // TODO: Create HighThroughputProcessor and process the stream
    let proc = HighThroughputProcessor::new(consumer_count);
    proc.process_market_stream(rx).await?;
    data_generation_task.await??;
    // TODO: Calculate elapsed time

    let elapsed = start_time.elapsed();
    // TODO: Print results
    let total_ticks = producer_count * ticks_per_producer;
    let throughput = f64::from_usize(total_ticks).unwrap() / elapsed.as_secs_f64();
    println!("\n=== Multiple Consumer Results ({consumer_count} consumers) ===");
    println!("total ticks: {total_ticks}");
    println!("Elapsed time: {elapsed:?}");
    println!("throughput: {throughput}");
    Ok(elapsed)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Market Data Processing Performance Test ===\n");

    // Test parameters
    let producer_count = 10;
    let ticks_per_producer = 10_000;
    let total_ticks = producer_count * ticks_per_producer;

    println!("Test setup:");
    println!("- {producer_count} producers");
    println!("- {ticks_per_producer} ticks per producer");
    println!("- {total_ticks} total ticks");

    // TODO: Run single consumer test
    let single_test = test_single_consumer(producer_count, ticks_per_producer).await?;
    // TODO: Add small delay between tests
    tokio::time::sleep(Duration::from_secs(5)).await;
    // TODO: Run multiple consumer test (try with 4 consumers)
    let multiple_test = test_multiple_consumers(producer_count, ticks_per_producer, 4).await?;
    // TODO: Compare and display results
    println!("\n=== Single Consumer Results ===\n");
    println!("Total time: {single_test:?}");
    println!("\n=== Multiple Consumer Results ===\n");
    println!("Total time: {multiple_test:?}");
    // let diff = single_test - multiple_test;
    println!("\n=== Comparison ===");
    // println!(
    //     "Single consumer took {:?} seconds longer",
    //     diff.as_secs_f64()
    // );
    let speedup = single_test.as_secs_f64() / multiple_test.as_secs_f64();
    println!("Speedup time: {speedup}");
    Ok(())
}
