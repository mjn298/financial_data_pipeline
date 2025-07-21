# Module 2: Concurrency with Channels

## Overview
This module introduces Rust's message-passing concurrency model using channels, focusing on producer-consumer patterns for financial data processing.

**Time Estimate:** 2-3 hours  
**Prerequisites:** Module 1 completed, understanding of async/await  
**Goal:** Build concurrent market data processing using channels instead of shared state

## Learning Objectives

By the end of this module, you will understand:
- Multi-producer, single-consumer (mpsc) channels
- Producer-consumer patterns for data processing
- Channel-based concurrency vs shared state
- Graceful shutdown and backpressure handling
- When to use channels vs other concurrency primitives

## Key Concepts

### 1. Message Passing Philosophy
- "Don't communicate by sharing memory; share memory by communicating"
- Channels transfer ownership of data between tasks
- Eliminates many data race conditions by design
- Natural fit for pipeline architectures

### 2. Channel Types
- **Unbounded channels**: Unlimited buffer, risk of memory growth
- **Bounded channels**: Fixed buffer size, provides backpressure
- **Oneshot channels**: Single message, useful for request-response
- **mpsc**: Multiple producers, single consumer (most common)

### 3. Producer-Consumer Patterns
- Multiple data sources (producers) feeding one processor (consumer)
- Buffering to handle different processing speeds
- Load balancing across multiple consumers
- Error handling and dead letter queues

### 4. Graceful Shutdown
- Signaling shutdown across multiple tasks
- Draining remaining messages before exit
- Timeout handling for stuck producers/consumers

## Exercises

### Exercise 2.1: Basic Channel Communication
Create a simple producer-consumer setup for market data:

```rust
use tokio::sync::mpsc;
use std::time::Duration;

pub struct MarketDataProducer {
    tx: mpsc::Sender<MarketTick>,
    symbol: String,
}

impl MarketDataProducer {
    pub fn new(tx: mpsc::Sender<MarketTick>, symbol: String) -> Self {
        // TODO: Implement constructor
    }
    
    pub async fn start_producing(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Generate and send market ticks every 50ms
        // TODO: Use fetch_market_data from Module 1
        // TODO: Handle channel closure gracefully
    }
}

pub struct MarketDataConsumer {
    rx: mpsc::Receiver<MarketTick>,
}

impl MarketDataConsumer {
    pub fn new(rx: mpsc::Receiver<MarketTick>) -> Self {
        // TODO: Implement constructor
    }
    
    pub async fn start_consuming(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Receive and process market ticks
        // TODO: Print tick details (symbol, price, volume, timestamp)
        // TODO: Calculate and print running average price per symbol
    }
}
```

**Tasks:**
1. Implement `MarketDataProducer` with periodic data generation
2. Implement `MarketDataConsumer` with message processing
3. Create a main function that spawns 3 producers (different symbols) and 1 consumer
4. Run for 10 seconds then gracefully shutdown

### Exercise 2.2: Multiple Consumers with Load Balancing
Extend the system to handle high-throughput scenarios:

```rust
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::time::Instant;

pub struct PriceAggregator {
    symbol_prices: std::collections::HashMap<String, Vec<Decimal>>,
    start_time: Instant,
}

impl PriceAggregator {
    pub fn new() -> Self {
        // TODO: Implement constructor
    }
    
    pub fn add_tick(&mut self, tick: MarketTick) {
        // TODO: Store price data for aggregation
    }
    
    pub fn get_statistics(&self, symbol: &str) -> Option<PriceStats> {
        // TODO: Calculate min, max, average, count for symbol
    }
    
    pub fn print_summary(&self) {
        // TODO: Print statistics for all symbols
    }
}

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
        // TODO: Implement constructor
    }
    
    pub async fn process_market_stream(
        &self,
        mut rx: mpsc::Receiver<MarketTick>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Spawn multiple consumer tasks
        // TODO: Each consumer processes ticks and updates shared aggregator
        // TODO: Coordinate graceful shutdown of all consumers
    }
}
```

**Tasks:**
1. Implement `PriceAggregator` for collecting statistics
2. Create multiple consumer tasks that share the aggregator
3. Use `Arc<Mutex<T>>` for shared state between consumers
4. Generate high-frequency data (10 producers, 1000 ticks/second total)
5. Process with 4 consumers and compare performance vs single consumer

### Exercise 2.3: Advanced Channel Patterns
Implement sophisticated patterns for production systems:

```rust
use tokio::sync::{mpsc, oneshot, broadcast};
use tokio::time::{timeout, Duration};

pub enum MarketCommand {
    Subscribe(String, oneshot::Sender<mpsc::Receiver<MarketTick>>),
    Unsubscribe(String),
    GetStats(oneshot::Sender<Vec<PriceStats>>),
    Shutdown,
}

pub struct MarketDataHub {
    command_tx: mpsc::Sender<MarketCommand>,
    subscribers: std::collections::HashMap<String, Vec<mpsc::Sender<MarketTick>>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl MarketDataHub {
    pub fn new() -> Self {
        // TODO: Implement constructor
    }
    
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Main event loop processing commands
        // TODO: Handle subscription management
        // TODO: Distribute market data to subscribers
        // TODO: Coordinate graceful shutdown
    }
    
    pub async fn subscribe_to_symbol(&self, symbol: String) -> Result<mpsc::Receiver<MarketTick>, Box<dyn std::error::Error>> {
        // TODO: Send subscription command and wait for response
    }
    
    pub async fn get_statistics(&self) -> Result<Vec<PriceStats>, Box<dyn std::error::Error>> {
        // TODO: Request statistics with timeout
    }
    
    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Initiate graceful shutdown
    }
}
```

**Tasks:**
1. Implement command-based architecture using channels
2. Support dynamic subscription/unsubscription to symbols
3. Use `oneshot` channels for request-response patterns
4. Use `broadcast` channels for shutdown signaling
5. Implement proper timeout handling and error recovery
6. Write integration tests for the complete system

## Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
rust_decimal = "1.33"
anyhow = "1.0"
rand = "0.8"

[dev-dependencies]
tokio-test = "0.4"
```

## Self-Check Questions

Before moving to Module 3, ensure you can answer:

1. **Channels vs Shared State:** When would you choose channels over `Arc<Mutex<T>>`? What are the trade-offs?

2. **Backpressure:** How do bounded channels provide backpressure? What happens when the buffer is full?

3. **Error Handling:** How do you handle producer failures without crashing consumers? What about consumer failures?

4. **Performance:** Why might multiple consumers be faster than a single consumer? When might they be slower?

5. **Graceful Shutdown:** How do you ensure all messages are processed before shutdown? What about stuck producers?

## Common Pitfalls

- **Unbounded channels**: Can lead to memory exhaustion under load
- **Forgetting to close senders**: Consumers will wait forever for more messages
- **Deadlocks with bounded channels**: Producer waiting for space, consumer not receiving
- **Clone vs move semantics**: Understanding when data is copied vs moved through channels
- **Error propagation**: Handling errors in producer tasks vs consumer tasks

## Performance Considerations

- **Channel capacity**: Tune based on producer/consumer speed differences
- **Task spawn overhead**: Balance between parallelism and task creation cost
- **Memory usage**: Monitor channel buffer sizes under different load patterns
- **CPU utilization**: Measure impact of context switching between tasks

## Next Steps

Once comfortable with these concepts:
- Benchmark your implementations with different channel sizes
- Experiment with different producer/consumer ratios
- Consider error handling strategies (retry, dead letter queues)
- Move on to Module 3: Shared State & Real Parallelism

## Resources

- [Tokio Channels Tutorial](https://tokio.rs/tokio/tutorial/channels)
- [Rust Book - Chapter 16 (Message Passing)](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
- [Async Book - Channels](https://rust-lang.github.io/async-book/05_streams/01_chapter.html)
- [tokio::sync Documentation](https://docs.rs/tokio/latest/tokio/sync/index.html)