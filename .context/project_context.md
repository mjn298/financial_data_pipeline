# Rust Financial Data Pipeline - Learning Project Context

## Project Overview

**Goal**: Learn Rust fundamentals by building a real-time financial data aggregation pipeline  
**Purpose**: Prepare for backend engineering interview at REDACTED  
**Timeline**: Working through 6 progressive modules at own pace

## What We're Building

A distributed financial data system with these components:
- **Data Ingester** - Simulates real-time market data feeds
- **Stream Processor** - Aggregates and enriches data using Kafka
- **Cache Layer** - Redis for hot data storage  
- **API Gateway** - Serves data via both gRPC and REST
- **Database** - PostgreSQL for persistent storage

## Learning Objectives

### Core Rust Concepts
- [ ] Basic syntax, structs, ownership, borrowing
- [ ] Parallelism and concurrency patterns
- [ ] Async/await and Tokio runtime
- [ ] Error handling with Result<T, E>
- [ ] Memory safety without garbage collection

### Distributed Systems Concepts  
- [ ] Message queues (Kafka) for event streaming
- [ ] Caching strategies (Redis) for performance
- [ ] Database integration (PostgreSQL) for persistence
- [ ] Load balancing and horizontal scaling
- [ ] Monitoring and observability

### API Design Patterns
- [ ] When to use gRPC vs REST
- [ ] Protocol buffers for type-safe APIs
- [ ] Streaming vs request-response patterns
- [ ] Authentication and authorization
- [ ] Rate limiting and backpressure

### Financial System Requirements
- [ ] Precise decimal arithmetic (never floats for money)
- [ ] Time zone handling for global markets
- [ ] Event sourcing and audit trails
- [ ] Low-latency processing for real-time data
- [ ] Compliance and regulatory considerations

## Tutorial Structure

### Module 1: Rust Basics & Async Foundations ⬅️ **CURRENT**
- **Status**: Setting up environment
- **Focus**: Structs, methods, ownership, basic async/await
- **Exercises**: MarketTick struct, async data fetching simulation
- **Time**: 1-2 hours

### Module 2: Concurrency with Channels  
- **Focus**: mpsc channels, producer-consumer patterns, concurrent tasks
- **Exercises**: Multiple market data producers, shared consumer
- **Key Concepts**: Message passing, avoiding shared state

### Module 3: Shared State & Real Parallelism
- **Focus**: Arc, Mutex, RwLock for thread-safe shared data
- **Exercises**: Thread-safe price aggregation, read-heavy workloads
- **Key Concepts**: When to share vs send, deadlock prevention

### Module 4: Network Communication - gRPC vs REST
- **Focus**: Protocol buffers, Tonic, Axum, performance trade-offs
- **Exercises**: Implement both APIs, benchmark performance
- **Key Concepts**: Type safety, streaming, HTTP/2 benefits

### Module 5: Distributed Components
- **Focus**: Kafka, Redis, PostgreSQL integration
- **Exercises**: Event streaming, cache-aside pattern, data persistence
- **Key Concepts**: Event-driven architecture, eventual consistency

### Module 6: Production Patterns
- **Focus**: Error handling, monitoring, graceful shutdown
- **Exercises**: Custom error types, retry logic, observability
- **Key Concepts**: Reliability, debugging, operational concerns

## Technical Stack

```toml
# Core dependencies for this project
[dependencies]
tokio = { version = "1.35", features = ["full"] }    # Async runtime
serde = { version = "1.0", features = ["derive"] }   # Serialization
chrono = { version = "0.4", features = ["serde"] }   # Time handling
uuid = { version = "1.6", features = ["v4"] }        # Unique IDs
rust_decimal = "1.33"                                # Precise arithmetic
anyhow = "1.0"                                       # Error handling

# Additional as we progress
tonic = "0.11"          # gRPC
axum = "0.7"           # REST API
rdkafka = "0.36"       # Kafka client
redis = "0.24"         # Redis client
sqlx = "0.7"           # PostgreSQL async driver
```

## Current Status

### ✅ Completed
- Project planning and learning path design
- Development environment setup guide
- Module 1 exercises and checklist created
- Dependencies and project structure defined

### 🔄 In Progress  
- **Module 1**: Working through Rust basics
  - Exercise 1.1: MarketTick struct implementation
  - Exercise 1.2: Async function with simulated market data fetch

### 📋 Next Steps
1. Complete Module 1 exercises
2. Verify understanding with self-check questions  
3. Move to Module 2 (channels and concurrency)
4. Build incrementally toward full pipeline

## Interview Preparation Context

**Target Role**: Backend Engineer at REDACTED  
**Key Areas**: Rust, parallelism/concurrency, gRPC vs REST, extreme scale architecture

**Why This Project Helps**:
- Demonstrates hands-on Rust experience beyond toy examples
- Shows understanding of distributed systems patterns
- Covers real financial data processing challenges
- Provides concrete examples for behavioral interview questions
- Builds portfolio project showcasing technical growth

## Development Notes

- Using RustRover IDE with built-in Rust support and debugging
- All code in `financial_data_pipeline/` directory
- Running `cargo check` frequently for fast feedback
- Using `dbg!` macro, RustRover debugger, and tests for debugging
- Committing progress at end of each module

## Getting Help

When asking questions or seeking help:
- Reference current module and specific exercise
- Include error messages and code snippets
- Mention which concepts are unclear
- Ask about trade-offs and alternative approaches

**Example**: "Working on Module 1 Exercise 1.2 - getting compiler error about missing .await on my async function. Here's my code..."