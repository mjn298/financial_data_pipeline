use crate::models::MarketTick;
use crate::processor::aggregator::{PriceAggregator, PriceStats};
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot, broadcast};
use tokio::time::{timeout, Duration};

/// Commands that can be sent to the MarketDataHub
/// This enum represents the command pattern - a way to encapsulate requests as objects
#[derive(Debug)]
pub enum MarketCommand {
    /// Subscribe to a symbol and get a receiver for market ticks
    /// Uses oneshot channel to send back the receiver to the client
    Subscribe(String, oneshot::Sender<mpsc::Receiver<MarketTick>>),
    
    /// Unsubscribe from a symbol (removes all subscribers for that symbol)
    Unsubscribe(String),
    
    /// Request current statistics for all symbols
    /// Uses oneshot channel for request-response pattern
    GetStats(oneshot::Sender<Vec<PriceStats>>),
    
    /// Signal graceful shutdown
    Shutdown,
}

/// Central hub that manages market data distribution and subscriptions
/// This demonstrates the actor pattern - a single task that owns state and processes messages
pub struct MarketDataHub {
    // Channel for receiving commands from clients
    command_tx: mpsc::Sender<MarketCommand>,
    command_rx: mpsc::Receiver<MarketCommand>,
    
    // Map of symbol -> list of subscribers (mpsc senders)
    // Each subscriber gets their own channel to receive market data
    subscribers: HashMap<String, Vec<mpsc::Sender<MarketTick>>>,
    
    // Broadcast channel for coordinating shutdown across all components
    shutdown_tx: broadcast::Sender<()>,
    shutdown_rx: broadcast::Receiver<()>,
    
    // Aggregator for collecting statistics
    aggregator: PriceAggregator,
    
    // Channel for receiving market data from producers
    data_rx: mpsc::Receiver<MarketTick>,
}

impl MarketDataHub {
    /// Create a new MarketDataHub
    /// Returns the hub and a command sender for clients to use
    pub fn new(data_rx: mpsc::Receiver<MarketTick>) -> (Self, mpsc::Sender<MarketCommand>) {
        // TODO: Create command channel with appropriate buffer size
        // TODO: Create broadcast channel for shutdown coordination
        // TODO: Initialize all fields and return hub + command sender
        todo!()
    }
    
    /// Main event loop - processes market data and commands concurrently
    /// Uses tokio::select! to handle multiple async operations
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Use tokio::select! to handle:
        //   1. Incoming market data from data_rx
        //   2. Commands from command_rx  
        //   3. Shutdown signals from shutdown_rx
        // TODO: Call appropriate handler methods for each case
        // TODO: Break loop on shutdown and send shutdown signal to subscribers
        todo!()
    }
    
    /// Process a market tick - add to aggregator and distribute to subscribers
    async fn process_market_tick(&mut self, tick: MarketTick) {
        // TODO: Add tick to aggregator for statistics
        // TODO: Find subscribers for this symbol
        // TODO: Send tick to all subscribers, removing closed channels
        // TODO: Handle full channels gracefully (log warning, don't block)
        todo!()
    }
    
    /// Handle subscription request - create new channel and add to subscribers
    async fn handle_subscribe(&mut self, symbol: String, response_tx: oneshot::Sender<mpsc::Receiver<MarketTick>>) {
        // TODO: Create new mpsc channel for this subscriber
        // TODO: Add sender to subscribers map for the symbol
        // TODO: Send receiver back to client via oneshot channel
        // TODO: Handle case where client dropped the oneshot receiver
        todo!()
    }
    
    /// Handle unsubscription - remove all subscribers for a symbol
    async fn handle_unsubscribe(&mut self, symbol: String) {
        // TODO: Remove all subscribers for the symbol
        // TODO: Log the unsubscription
        todo!()
    }
    
    /// Handle statistics request - collect stats and send via oneshot
    async fn handle_get_stats(&self, response_tx: oneshot::Sender<Vec<PriceStats>>) {
        // TODO: Collect statistics for all symbols with subscribers
        // TODO: Send stats back via oneshot channel
        // TODO: Handle case where client dropped the oneshot receiver
        todo!()
    }
    
    /// Client API: Subscribe to a symbol (returns receiver for market ticks)
    /// This is the public API that clients use - it sends a command and waits for response
    pub async fn subscribe_to_symbol(&self, symbol: String) -> Result<mpsc::Receiver<MarketTick>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Create oneshot channel for response
        // TODO: Send Subscribe command to hub
        // TODO: Wait for response with timeout (use tokio::time::timeout)
        // TODO: Return the receiver or error
        todo!()
    }
    
    /// Client API: Get current statistics for all symbols
    pub async fn get_statistics(&self) -> Result<Vec<PriceStats>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Create oneshot channel for response
        // TODO: Send GetStats command to hub
        // TODO: Wait for response with timeout
        // TODO: Return stats or error
        todo!()
    }
    
    /// Client API: Unsubscribe from a symbol
    pub async fn unsubscribe_from_symbol(&self, symbol: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Send Unsubscribe command to hub
        // TODO: Handle send errors
        todo!()
    }
    
    /// Client API: Initiate graceful shutdown
    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Send Shutdown command to hub
        // TODO: Handle send errors
        todo!()
    }
    
    /// Get a receiver for shutdown notifications
    /// Clients can use this to coordinate their own shutdown
    pub fn subscribe_to_shutdown(&self) -> broadcast::Receiver<()> {
        // TODO: Return a new receiver from the broadcast channel
        todo!()
    }
}

// Key learning points for this exercise:
//
// 1. **Command Pattern**: MarketCommand enum encapsulates different operations
//    - Enables type-safe message passing between components
//    - Each command carries its own data and response channel
//
// 2. **Actor Pattern**: MarketDataHub owns all state and processes messages sequentially
//    - Eliminates data races by design (no shared mutable state)
//    - Single task handles all operations, making reasoning easier
//
// 3. **Channel Types**:
//    - mpsc: Multi-producer, single-consumer (commands, market data)
//    - oneshot: Single message request-response (subscribe, get stats)  
//    - broadcast: One-to-many messaging (shutdown coordination)
//
// 4. **Error Handling**:
//    - Timeout on responses to prevent hanging
//    - Graceful handling of closed channels
//    - Proper cleanup on shutdown
//
// 5. **Concurrent Patterns**:
//    - tokio::select! for handling multiple async operations
//    - Non-blocking sends with proper error handling
//    - Coordinated shutdown across multiple components