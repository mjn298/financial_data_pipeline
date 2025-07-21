// Example demonstrating Exercise 2.3: Advanced Channel Patterns
// This shows how to use the MarketDataHub with dynamic subscriptions

use financial_data_pipeline::processor::{MarketDataProducer, MarketDataHub};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use financial_data_pipeline::MarketTick;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Hub Example - Exercise 2.3");
    
    // TODO: Create the market data channel that will feed the hub
    // This channel carries MarketTick messages from producers to the hub
    let (tx, rx) = mpsc::channel::<MarketTick>(1000);


    // TODO: Create the MarketDataHub instance
    // The constructor should return (hub, command_sender)
    // The command_sender is what clients use to interact with the hub
    
    // TODO: Spawn the hub in a background task
    // The hub.start() method runs the main event loop
    // This task will handle all market data distribution and commands
    
    // TODO: Create multiple producers for different symbols
    // Each producer should send MarketTick messages to data_tx
    // Spawn each producer in its own task for concurrent data generation
    
    // TODO: Create a client task that demonstrates dynamic subscription patterns
    // The client should:
    //   1. Subscribe to one symbol and receive some ticks
    //   2. Subscribe to additional symbols
    //   3. Receive ticks from multiple symbols concurrently (use tokio::select!)
    //   4. Request statistics from the hub
    //   5. Unsubscribe from a symbol
    //   6. Verify no more data comes from unsubscribed symbol
    //   7. Initiate graceful shutdown
    
    // TODO: Wait for the client task to complete
    
    // TODO: Wait for the hub task to shutdown cleanly
    
    println!("Hub Example completed successfully!");
    
    Ok(())
}

// Additional example functions you should consider implementing:

/// Example of multiple clients subscribing to different symbols
async fn _multi_client_example() {
    // TODO: Create multiple client tasks, each subscribing to different symbols
    // TODO: Show how the hub can distribute the same data to multiple subscribers
    // TODO: Demonstrate that each client gets their own independent stream
    // TODO: Show concurrent subscription/unsubscription patterns
}

/// Example of error handling and recovery
async fn _error_handling_example() {
    // TODO: Test what happens when a client drops their receiver
    // TODO: Test timeout behavior when hub is overloaded
    // TODO: Test graceful handling of invalid symbol requests
    // TODO: Show how the system recovers from temporary errors
}

/// Example of performance testing
async fn _performance_example() {
    // TODO: Create high-volume producers (measure ticks/second)
    // TODO: Multiple subscribers per symbol to test distribution overhead
    // TODO: Measure end-to-end latency from producer to subscriber
    // TODO: Monitor memory usage with different subscriber counts
}