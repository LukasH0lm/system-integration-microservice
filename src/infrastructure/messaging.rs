use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

// This defines the signature for our async handler functions.
type HandlerFn = Box<dyn Fn(Vec<u8>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

pub struct MessageDispatcher {
    handlers: HashMap<String, HandlerFn>,
}

impl MessageDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    // This dynamically registers handlers by their routing key
    pub fn register<F, Fut>(&mut self, event_name: &str, handler: F)
    where
        F: Fn(Vec<u8>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let boxed_handler = Box::new(move |payload| {
            Box::pin(handler(payload)) as Pin<Box<dyn Future<Output = ()> + Send>>
        });

        self.handlers.insert(event_name.to_string(), boxed_handler);
    }

    // This dispatches an incoming message to the correct handler
    pub async fn dispatch(&self, event_name: &str, payload: Vec<u8>) {
        if let Some(handler) = self.handlers.get(event_name) {
            handler(payload).await;
        } else {
            println!("No handler registered for event: {}", event_name);
        }
    }
}
