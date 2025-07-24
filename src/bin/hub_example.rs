// Example demonstrating Exercise 2.3: Advanced Channel Patterns
// This shows how to use the MarketDataHub with dynamic subscriptions

use financial_data_pipeline::MarketTick;
use financial_data_pipeline::processor::{
    MarketCommand, MarketDataHub, MarketDataProducer, PriceStats,
};
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Hub Example - Exercise 2.3");

    // This channel carries MarketTick messages from producers to the hub
    let (tx, rx) = mpsc::channel::<MarketTick>(1000);

    // The constructor should return (hub, command_sender)
    // The command_sender is what clients use to interact with the hub
    let mut hub = MarketDataHub::new(rx);
    let command_sender = hub.get_command_sender();

    // The hub.start() method runs the main event loop
    // This task will handle all market data distribution and commands
    let hub_task = tokio::spawn(async move { hub.start().await });

    // Each producer should send MarketTick messages to data_tx
    // Spawn each producer in its own task for concurrent data generation
    let symbols = vec!["VZW", "JNJ", "AMZN", "AAPL", "SONO"];
    for symbol in symbols {
        let producer_tx = tx.clone();
        let symbol = symbol.to_string();

        tokio::spawn(async move {
            let producer = MarketDataProducer::new(producer_tx, symbol);
            producer.start_producing().await.unwrap();
        });
    }
    // The client should:
    //   1. Subscribe to one symbol and receive some ticks
    //   2. Subscribe to additional symbols
    //   3. Receive ticks from multiple symbols concurrently (use tokio::select!)
    //   4. Request statistics from the hub
    //   5. Unsubscribe from a symbol
    //   6. Verify no more data comes from unsubscribed symbol
    //   7. Initiate graceful shutdown
    let client_command_sender = command_sender.clone();

    let client_task = tokio::spawn(async move {
        let (tx, rx) = oneshot::channel::<mpsc::Receiver<MarketTick>>();
        client_command_sender
            .send(MarketCommand::Subscribe("VZW".to_string(), tx))
            .await
            .expect("Failed to send subscribe command");
        let mut vzw_receiver = rx.await.expect("Failed to get receiver for VZW");
        for i in 0..5 {
            if let Some(tick) = vzw_receiver.recv().await {
                println!("Received tick {i}: {tick:?}")
            }
        }

        let (jnj_tx, jnj_rx) = oneshot::channel::<mpsc::Receiver<MarketTick>>();
        client_command_sender
            .send(MarketCommand::Subscribe("JNJ".to_string(), jnj_tx))
            .await
            .expect("Failed to send subscribe command");

        let mut jnj_receiver = jnj_rx.await.expect("Failed to get receiver for JNJ");

        for i in 0..1000 {
            tokio::select! {
                Some(tick) = jnj_receiver.recv() => {
                    println!("JNJ tick {i}: {tick:?}");
                },
                Some(tick) = vzw_receiver.recv() => {
                    println!("VZW tick {i}: {tick:?}");
                }
            }
        }

        let (oneshot_sender, oneshot_receiver) = oneshot::channel::<Vec<PriceStats>>();
        client_command_sender
            .send(MarketCommand::GetStats(oneshot_sender))
            .await
            .expect("Failed to send GetStats command");
        let stats = oneshot_receiver.await.expect("Failed to get stats");
        println!("Statistics: {stats:?}");

        client_command_sender
            .send(MarketCommand::Unsubscribe("VZW".to_string()))
            .await
            .expect("Failed to unsubscribe");

        for _ in 0..10 {
            tokio::select! {
                Some(tick) = vzw_receiver.recv() => {
                    println!("ERROR: Still receiving VZW after unsubscribe: {tick:?}");
                }
                Some(tick) = jnj_receiver.recv() => {
                    println!("JNJ tick (ok): {tick:?}");
                }
                else => {
                    println!("No data received")
                }
            }
        }

        client_command_sender
            .send(MarketCommand::Shutdown)
            .await
            .expect("Failed to shutdown");
    });
    client_task.await?;
    hub_task.await?.expect("Failed to shutdown hub!");

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
