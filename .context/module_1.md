# Module 1: Rust Basics & Async Foundations

## Overview
This module introduces core Rust concepts and async programming fundamentals through practical financial data exercises.

**Time Estimate:** 1-2 hours  
**Prerequisites:** Rust installed, basic programming experience  
**Goal:** Build foundation for async financial data processing

## Getting Started

### 1. Create a New Rust Project

First, create a new Rust library crate for your financial data pipeline:

```bash
cargo new --lib financial_data_pipeline
cd financial_data_pipeline
```

This creates a new directory `financial_data_pipeline/` with:
- `Cargo.toml` - Project configuration and dependencies
- `src/lib.rs` - Main library file where you'll write your code

### 2. Set Up Your Workspace

Replace the contents of `src/lib.rs` with:

```rust
//! Financial Data Pipeline - Module 1
//! 
//! This module demonstrates Rust basics and async programming
//! for financial data processing.

pub mod market_data;

pub use market_data::*;
```

Create `src/market_data.rs` where you'll implement the exercises:

```rust
// This is where you'll implement your MarketTick struct and async functions
```

## Learning Objectives

By the end of this module, you will understand:
- Rust struct definitions and method implementations
- Ownership, borrowing, and memory safety concepts
- Basic async/await syntax and usage
- Error handling with `Result<T, E>`
- Working with external crates for time and serialization

## Key Concepts

### 1. Structs and Methods
- Defining data structures with `struct`
- Implementing methods with `impl` blocks
- Public vs private fields and methods
- Derived traits like `Debug`, `Clone`

### 2. Ownership and Borrowing
- Move semantics by default
- Borrowing with `&` (immutable) and `&mut` (mutable)
- Lifetime management without garbage collection
- When to clone vs borrow

### 3. Async Programming Basics
- `async fn` syntax
- `.await` for waiting on futures
- Basic Tokio runtime usage
- Understanding when operations should be async

### 4. Error Handling
- `Result<T, E>` type for fallible operations
- Using `?` operator for error propagation
- `anyhow` crate for simplified error handling

## Exercises

### Exercise 1.1: MarketTick Struct
Create a `MarketTick` struct to represent financial market data:

```rust
use chrono::{DateTime, Utc};
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
        // TODO: Implement constructor
    }
    
    pub fn is_significant_volume(&self) -> bool {
        // TODO: Return true if volume > 1000
    }
}
```

**Tasks:**
1. Complete the `new` method (use `Utc::now()` for timestamp)
2. Implement `is_significant_volume` method
3. Write tests for both methods

### Exercise 1.2: Async Data Fetching
Simulate fetching market data with async functions:

```rust
use tokio::time::{sleep, Duration};

pub async fn fetch_market_data(symbol: &str) -> Result<MarketTick, String> {
    // TODO: Simulate network delay with sleep(Duration::from_millis(100))
    // TODO: Return a MarketTick with random price and volume
    // TODO: Return Err for symbol "INVALID"
}

pub async fn fetch_multiple_symbols(symbols: Vec<&str>) -> Vec<MarketTick> {
    // TODO: Fetch data for all symbols concurrently
    // TODO: Filter out any errors
}
```

**Tasks:**
1. Implement `fetch_market_data` with simulated delay
2. Use `rand` crate for random price/volume generation
3. Implement `fetch_multiple_symbols` using concurrent futures
4. Write async tests using `#[tokio::test]`

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

Before moving to Module 2, ensure you can answer:

1. **Ownership:** Why does Rust move values by default? When would you use `.clone()`?

2. **Borrowing:** What's the difference between `&self` and `&mut self` in method signatures?

3. **Async:** When should a function be `async`? What does `.await` actually do?

4. **Error Handling:** How does `?` work with `Result` types? When would you use `unwrap()`?

5. **Memory Safety:** How does Rust prevent data races without garbage collection?

## Common Pitfalls

- **Forgetting `.await`:** Async functions return futures that must be awaited
- **Fighting the borrow checker:** Start with cloning, optimize later
- **Using `f64` for money:** Always use `Decimal` for financial calculations
- **Blocking in async context:** Use async versions of I/O operations

## Next Steps

Once comfortable with these concepts:
- Review your solutions with a focus on error handling
- Run `cargo clippy` for additional suggestions
- Consider performance implications of your choices
- Move on to Module 2: Concurrency with Channels

## Resources

- [The Rust Book - Chapter 4 (Ownership)](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Async Book - Getting Started](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Decimal Documentation](https://docs.rs/rust_decimal/latest/rust_decimal/)