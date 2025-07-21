# Financial Data Pipeline - Complete Project Setup

## Project Structure

```
financial-data-pipeline/
├── README.md                     # Project overview and setup
├── PROJECT_CONTEXT.md            # Learning context and progress tracking
├── TUTORIAL.md                   # Complete tutorial guide
├── SETUP_GUIDE.md               # Environment setup instructions
├── .gitignore                   # Rust gitignore
├── Cargo.toml                   # Project dependencies
├── Cargo.lock                   # Dependency lock file (auto-generated)
├── docker-compose.yml           # Local infrastructure (Kafka, Redis, PostgreSQL)
├── proto/                       # Protocol buffer definitions
│   └── market_data.proto
├── src/                         # Source code
│   ├── main.rs                  # Application entry point
│   ├── lib.rs                   # Library root
│   ├── models/                  # Data structures
│   │   ├── mod.rs
│   │   ├── market_tick.rs
│   │   └── aggregated_data.rs
│   ├── ingester/               # Module 1-2: Data ingestion
│   │   ├── mod.rs
│   │   ├── simulator.rs
│   │   └── kafka_producer.rs
│   ├── processor/              # Module 3: Stream processing
│   │   ├── mod.rs
│   │   ├── aggregator.rs
│   │   └── kafka_consumer.rs
│   ├── storage/                # Module 5: Data persistence
│   │   ├── mod.rs
│   │   ├── postgres.rs
│   │   └── redis.rs
│   ├── api/                    # Module 4: API endpoints
│   │   ├── mod.rs
│   │   ├── grpc_server.rs
│   │   └── rest_server.rs
│   └── config/                 # Module 6: Configuration
│       ├── mod.rs
│       └── settings.rs
├── tests/                      # Integration tests
│   ├── integration_test.rs
│   └── common/
│       └── mod.rs
├── benches/                    # Performance benchmarks
│   └── market_data_bench.rs
├── docs/                       # Additional documentation
│   ├── architecture.md
│   ├── module_guides/
│   │   ├── module_1.md
│   │   ├── module_2.md
│   │   ├── module_3.md
│   │   ├── module_4.md
│   │   ├── module_5.md
│   │   └── module_6.md
│   └── images/
└── scripts/                    # Utility scripts
    ├── setup.sh                # Environment setup
    ├── start_infrastructure.sh # Start Docker services
    └── test_all.sh             # Run all tests
```

## File Contents

### Root Files

#### README.md
```markdown
# Financial Data Pipeline

A real-time financial data aggregation system built in Rust, designed as a learning project for backend engineering and distributed systems concepts.

## Features

- **Real-time data ingestion** with simulated market feeds
- **Stream processing** with Kafka for event-driven architecture
- **Dual API support** with both gRPC and REST endpoints
- **Multi-tier storage** with Redis caching and PostgreSQL persistence
- **Production patterns** including error handling, monitoring, and graceful shutdown

## Quick Start

1. **Prerequisites**: Rust, Docker, Docker Compose
2. **Clone and setup**:
   ```bash
   git clone <repo-url>
   cd financial-data-pipeline
   cargo check  # Verify Rust setup
   ```
3. **Start infrastructure**:
   ```bash
   ./scripts/start_infrastructure.sh
   ```
4. **Run the application**:
   ```bash
   cargo run
   ```

## Learning Modules

This project is structured as a progressive tutorial:

- **Module 1**: Rust basics and async foundations
- **Module 2**: Concurrency with channels
- **Module 3**: Shared state and parallelism
- **Module 4**: gRPC vs REST APIs
- **Module 5**: Distributed components (Kafka, Redis, PostgreSQL)
- **Module 6**: Production patterns

See `TUTORIAL.md` for detailed learning path and `docs/module_guides/` for individual module instructions.

## Architecture

```
[Data Sources] -> [Kafka] -> [Stream Processor] -> [Database]
                                    |
                                    v
                               [Redis Cache]
                                    |
                                    v
                            [API Gateway]
                           /              \
                    [gRPC Server]    [REST Server]
```

## Development

- **IDE**: RustRover (or VS Code with rust-analyzer)
- **Testing**: `cargo test`
- **Benchmarks**: `cargo bench`
- **Linting**: `cargo clippy`
- **Formatting**: `cargo fmt`

## Contributing

This is a learning project, but PRs welcome for:
- Bug fixes
- Documentation improvements
- Additional learning exercises
- Performance optimizations

## License

MIT License - see LICENSE file for details.
```

#### .gitignore
```gitignore
# Rust
/target/
Cargo.lock

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Logs
*.log

# Environment
.env
.env.local

# Database
*.db
*.sqlite

# Temporary files
*.tmp
*.temp
```

#### Cargo.toml
```toml
[package]
name = "financial-data-pipeline"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "Real-time financial data aggregation pipeline for learning Rust and distributed systems"
license = "MIT"
keywords = ["finance", "rust", "async", "distributed-systems", "learning"]

[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Time handling
chrono = { version = "0.4", features = ["serde"] }

# Unique identifiers
uuid = { version = "1.6", features = ["v4", "serde"] }

# Random generation
rand = "0.8"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Configuration
config = "0.13"

# Financial calculations
rust_decimal = { version = "1.33", features = ["serde"] }

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# gRPC
tonic = "0.11"
prost = "0.12"

# REST API
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid", "rust_decimal"] }

# Cache
redis = { version = "0.24", features = ["tokio-comp"] }

# Message queue
rdkafka = { version = "0.36", features = ["cmake-build"] }

[build-dependencies]
tonic-build = "0.11"

[dev-dependencies]
tokio-test = "0.4"

[[bench]]
name = "market_data_bench"
harness = false

[profile.release]
debug = 1  # Include debug info for profiling
```

#### docker-compose.yml
```yaml
version: '3.8'

services:
  zookeeper:
    image: confluentinc/cp-zookeeper:latest
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
      ZOOKEEPER_TICK_TIME: 2000
    ports:
      - "2181:2181"

  kafka:
    image: confluentinc/cp-kafka:latest
    depends_on:
      - zookeeper
    ports:
      - "9092:9092"
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://localhost:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data

  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: financial_data
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./scripts/init.sql:/docker-entrypoint-initdb.d/init.sql

volumes:
  redis_data:
  postgres_data:
```

### Source Code Structure

#### src/main.rs
```rust
//! Financial Data Pipeline
//! 
//! A learning project for Rust, async programming, and distributed systems.
//! Start with Module 1 exercises in the TUTORIAL.md file.

use anyhow::Result;
use tracing::info;

mod config;
mod models;

// Modules for different learning phases
#[cfg(feature = "module1")]
mod ingester;

#[cfg(feature = "module2")]
mod processor;

#[cfg(feature = "module4")]
mod api;

#[cfg(feature = "module5")]
mod storage;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting Financial Data Pipeline");
    
    // TODO: This will be filled in as you progress through modules
    println!("Welcome to the Financial Data Pipeline!");
    println!("Check TUTORIAL.md to get started with Module 1");
    
    Ok(())
}
```

#### src/lib.rs
```rust
//! Financial Data Pipeline Library
//! 
//! This library provides the core components for building a real-time
//! financial data processing system.

pub mod models;
pub mod config;

#[cfg(feature = "module1")]
pub mod ingester;

#[cfg(feature = "module2")]
pub mod processor;

#[cfg(feature = "module4")]
pub mod api;

#[cfg(feature = "module5")]
pub mod storage;

// Re-export common types
pub use models::*;
pub use config::*;
```

#### src/models/mod.rs
```rust
//! Data models for financial market data

mod market_tick;
mod aggregated_data;

pub use market_tick::*;
pub use aggregated_data::*;
```

#### src/models/market_tick.rs
```rust
//! Market tick data structure
//! 
//! This is your starting point for Module 1, Exercise 1.1

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTick {
    // TODO: Module 1, Exercise 1.1 - Add fields here:
    // - id: Uuid
    // - symbol: String  
    // - price: rust_decimal::Decimal (not f64!)
    // - volume: u64
    // - timestamp: DateTime<Utc>
    // - exchange: String
}

impl MarketTick {
    // TODO: Module 1, Exercise 1.1 - Implement constructor
    pub fn new(symbol: String, price: f64, volume: u64, exchange: String) -> Self {
        todo!("Implement this in Module 1, Exercise 1.1")
    }
    
    // TODO: Module 1, Exercise 1.1 - Implement price check
    pub fn is_expensive(&self) -> bool {
        todo!("Return true if price > $100")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_market_tick_creation() {
        // TODO: Add tests as you implement the struct
    }
}
```

### Scripts

#### scripts/setup.sh
```bash
#!/bin/bash
set -e

echo "Setting up Financial Data Pipeline development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust not found. Please install Rust first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "Docker not found. Please install Docker first."
    exit 1
fi

# Install additional Rust components
rustup component add clippy rustfmt

# Create necessary directories
mkdir -p logs
mkdir -p data

# Set up git hooks (optional)
if [ -d ".git" ]; then
    echo "Setting up git hooks..."
    echo "#!/bin/sh\ncargo fmt && cargo clippy -- -D warnings" > .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
fi

echo "Setup complete! Run 'cargo check' to verify everything works."
echo "Then start with Module 1 in TUTORIAL.md"
```

#### scripts/start_infrastructure.sh
```bash
#!/bin/bash
set -e

echo "Starting infrastructure services..."

# Start Docker services
docker-compose up -d

# Wait for services to be ready
echo "Waiting for services to start..."
sleep 10

# Check if Kafka is ready
echo "Checking Kafka..."
docker-compose exec kafka kafka-topics --bootstrap-server localhost:9092 --list > /dev/null || {
    echo "Kafka not ready, waiting longer..."
    sleep 20
}

# Create Kafka topics
echo "Creating Kafka topics..."
docker-compose exec kafka kafka-topics --create --bootstrap-server localhost:9092 --topic market-ticks --partitions 3 --replication-factor 1 --if-not-exists

echo "Infrastructure is ready!"
echo "- Kafka: localhost:9092"
echo "- Redis: localhost:6379" 
echo "- PostgreSQL: localhost:5432"
echo ""
echo "Run 'cargo run' to start the application"
```

## Getting Started Commands

```bash
# 1. Create the project structure
mkdir financial-data-pipeline
cd financial-data-pipeline

# 2. Initialize git repository
git init

# 3. Create all the files above
# (You can copy/paste each file content)

# 4. Initialize Cargo project
cargo init --lib

# 5. Replace Cargo.toml with the content above

# 6. Set up the directory structure
mkdir -p src/{models,ingester,processor,storage,api,config}
mkdir -p proto docs/module_guides tests benches scripts

# 7. Make scripts executable
chmod +x scripts/*.sh

# 8. Verify setup
cargo check

# 9. Start learning!
# Open TUTORIAL.md and begin with Module 1
```

This creates a complete, professional project structure that grows with your learning. Each module adds new functionality to existing files rather than starting from scratch.