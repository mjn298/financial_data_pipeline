# Module 3: Shared State & Real Parallelism

## Overview
This module explores shared state concurrency using Arc, Mutex, and RwLock, contrasting with the message-passing approach from Module 2. You'll learn when to use shared state vs channels and how to achieve real parallelism for CPU-bound tasks.

**Time Estimate:** 3-4 hours  
**Prerequisites:** Module 2 completed, understanding of channels and async  
**Goal:** Master thread-safe shared state patterns and parallel data processing

## Learning Objectives

By the end of this module, you will understand:
- Arc (Atomic Reference Counting) for sharing ownership across threads
- Mutex vs RwLock: choosing the right synchronization primitive
- Deadlock prevention and lock ordering strategies
- CPU-bound vs IO-bound parallelism patterns
- Performance implications of different concurrency approaches

## Key Concepts

### 1. Shared State vs Message Passing
- **Channels**: Transfer ownership, no shared memory
- **Shared State**: Multiple threads access same memory
- **Trade-offs**: Performance vs complexity
- **When to use each**: Depends on access patterns and data flow

### 2. Arc<T> - Shared Ownership
- Reference counting across threads
- Immutable access to shared data
- Clone is cheap (increments counter)
- Automatically cleaned up when count reaches zero

### 3. Mutex<T> - Mutual Exclusion
- Exclusive access to shared data
- Lock must be acquired before access
- Automatically released when guard dropped
- Can cause contention under high load

### 4. RwLock<T> - Reader-Writer Lock
- Multiple readers OR single writer
- Better for read-heavy workloads
- More complex than Mutex
- Can starve writers in extreme cases

### 5. Real Parallelism
- `std::thread` for CPU-bound work
- Thread pools with `rayon` or manual implementation
- Work stealing and load balancing
- Measuring parallel speedup

## Exercises

### Exercise 3.1: Thread-Safe Market Data Cache
Build a high-performance cache for market data using Arc and RwLock:

```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct MarketDataCache {
    data: Arc<RwLock<HashMap<String, CachedTick>>>,
    ttl: Duration,
}

#[derive(Clone)]
pub struct CachedTick {
    tick: MarketTick,
    inserted_at: Instant,
}

impl MarketDataCache {
    pub fn new(ttl: Duration) -> Self {
        // TODO: Initialize with empty cache and TTL
    }
    
    pub fn insert(&self, tick: MarketTick) {
        // TODO: Acquire write lock and insert
        // TODO: Consider implementing size limits
    }
    
    pub fn get(&self, symbol: &str) -> Option<MarketTick> {
        // TODO: Acquire read lock
        // TODO: Check if data exists and not expired
        // TODO: Return cloned tick if valid
    }
    
    pub fn get_all_valid(&self) -> Vec<MarketTick> {
        // TODO: Read all non-expired entries
        // TODO: Demonstrate read parallelism
    }
    
    pub fn cleanup_expired(&self) {
        // TODO: Remove expired entries
        // TODO: Minimize write lock duration
    }
    
    pub fn get_stats(&self) -> CacheStats {
        // TODO: Return cache statistics
        // TODO: Count entries, expired, etc.
    }
}

pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub unique_symbols: usize,
}
```

**Tasks:**
1. Implement thread-safe cache with TTL
2. Optimize for concurrent reads
3. Implement periodic cleanup task
4. Benchmark read vs write performance
5. Compare with channel-based approach

### Exercise 3.2: Parallel Price Calculator
Implement CPU-intensive calculations using real threads:

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam::channel;

pub struct ParallelPriceCalculator {
    thread_count: usize,
    work_queue: Arc<Mutex<Vec<CalculationTask>>>,
}

pub struct CalculationTask {
    pub symbol: String,
    pub prices: Vec<Decimal>,
    pub calculation_type: CalculationType,
}

pub enum CalculationType {
    MovingAverage(usize),    // Window size
    Volatility,              // Standard deviation
    RSI(usize),             // Relative Strength Index
    BollingerBands(usize),  // Period
}

pub struct CalculationResult {
    pub symbol: String,
    pub calculation_type: CalculationType,
    pub result: Decimal,
    pub computation_time: Duration,
}

impl ParallelPriceCalculator {
    pub fn new(thread_count: usize) -> Self {
        // TODO: Initialize calculator with thread pool size
    }
    
    pub fn calculate_batch(&self, tasks: Vec<CalculationTask>) -> Vec<CalculationResult> {
        // TODO: Distribute tasks across threads
        // TODO: Use channels or shared queue
        // TODO: Collect results maintaining order
        // TODO: Handle errors gracefully
    }
    
    fn moving_average(prices: &[Decimal], window: usize) -> Decimal {
        // TODO: Implement efficient moving average
        // TODO: Make this CPU-intensive for demonstration
    }
    
    fn calculate_volatility(prices: &[Decimal]) -> Decimal {
        // TODO: Calculate standard deviation
        // TODO: Add artificial complexity for benchmarking
    }
    
    fn calculate_rsi(prices: &[Decimal], period: usize) -> Decimal {
        // TODO: Implement RSI calculation
        // TODO: Demonstrate parallel gain/loss calculation
    }
}

pub struct ThreadPoolCalculator {
    // TODO: Implement custom thread pool
    // TODO: Work stealing queue
    // TODO: Graceful shutdown
}
```

**Tasks:**
1. Implement parallel calculation engine
2. Compare different work distribution strategies
3. Measure speedup with different thread counts
4. Implement work stealing for load balancing
5. Profile CPU utilization and bottlenecks

### Exercise 3.3: Lock-Free Data Structures
Explore atomic operations and lock-free patterns:

```rust
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use crossbeam::queue::SegQueue;

pub struct LockFreeMetrics {
    tick_count: Arc<AtomicU64>,
    total_volume: Arc<AtomicU64>,
    is_active: Arc<AtomicBool>,
    price_updates: Arc<SegQueue<PriceUpdate>>,
}

pub struct PriceUpdate {
    pub symbol: String,
    pub old_price: Decimal,
    pub new_price: Decimal,
    pub timestamp: Instant,
}

impl LockFreeMetrics {
    pub fn new() -> Self {
        // TODO: Initialize atomic counters
    }
    
    pub fn record_tick(&self, tick: &MarketTick) {
        // TODO: Atomically increment counters
        // TODO: Use appropriate ordering
    }
    
    pub fn record_price_change(&self, update: PriceUpdate) {
        // TODO: Add to lock-free queue
        // TODO: Limit queue size
    }
    
    pub fn get_snapshot(&self) -> MetricsSnapshot {
        // TODO: Read all metrics atomically
        // TODO: Ensure consistency
    }
    
    pub fn reset(&self) {
        // TODO: Atomically reset all counters
        // TODO: Clear queue
    }
}

pub struct HighFrequencyOrderBook {
    // TODO: Implement lock-free order book
    // TODO: Compare performance with locked version
    // TODO: Handle ABA problem
}
```

**Tasks:**
1. Implement lock-free metrics collection
2. Use atomic operations correctly
3. Compare with mutex-based approach
4. Implement lock-free queue operations
5. Benchmark under high contention

## Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
rust_decimal = "1.33"
anyhow = "1.0"
crossbeam = "0.8"  # For advanced concurrent data structures
parking_lot = "0.12"  # Faster mutex implementation
rayon = "1.8"  # Data parallelism library
num_cpus = "1.16"  # Detect CPU count

[dev-dependencies]
criterion = "0.5"  # Benchmarking
```

## Self-Check Questions

Before moving to Module 4, ensure you can answer:

1. **Arc vs Rc:** Why can't we use `Rc` in multi-threaded contexts? What makes `Arc` thread-safe?

2. **Mutex vs RwLock:** When would you choose `RwLock` over `Mutex`? What are the downsides?

3. **Deadlocks:** How can you prevent deadlocks when using multiple locks? What is lock ordering?

4. **Performance:** When is shared state faster than channels? When is it slower?

5. **Atomics:** What memory orderings exist and when to use each? What is the ABA problem?

## Common Pitfalls

- **Lock Contention**: Holding locks too long ruins parallelism
- **Deadlocks**: Always acquire multiple locks in same order
- **Over-sharing**: Not everything needs to be shared
- **Wrong Primitive**: Using Mutex for read-heavy workloads
- **Poisoned Locks**: Handling panics while holding locks

## Performance Considerations

- **Lock Granularity**: Fine-grained vs coarse-grained locking
- **Reader-Writer Ratios**: Measure actual access patterns
- **Cache Effects**: False sharing and cache line bouncing
- **Thread Pool Sizing**: Optimal thread count varies by workload
- **Atomic Operations**: Not always faster than locks

## Integration Challenge

Combine channels and shared state for a hybrid approach:
- Use channels for task distribution
- Use shared state for results aggregation
- Implement backpressure and flow control
- Monitor performance metrics in real-time

## Next Steps

Once comfortable with these concepts:
- Benchmark your implementations under various loads
- Try implementing custom synchronization primitives
- Explore wait-free algorithms
- Move on to Module 4: Network Communication (gRPC vs REST)

## Resources

- [Rust Book - Chapter 16 (Shared State)](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Rustonomicon - Atomics](https://doc.rust-lang.org/nomicon/atomics.html)
- [crossbeam documentation](https://docs.rs/crossbeam/latest/crossbeam/)
- [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) - Lock-free data structures
- [Rayon documentation](https://docs.rs/rayon/latest/rayon/)