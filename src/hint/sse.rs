use tokio::sync::broadcast::{self, error::SendError, Receiver, Sender};

use super::sanitizer::SanitizedEvent;

#[derive(Clone, Debug)]
pub struct SseBroadcaster {
    sender: Sender<SanitizedEvent>,
}

impl SseBroadcaster {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity.max(1));
        Self { sender }
    }

    pub fn publish(&self, event: SanitizedEvent) -> Result<usize, &'static str> {
        self.sender
            .send(event)
            .map_err(|SendError(_)| "no active subscribers")
    }

    pub fn subscribe(&self) -> Receiver<SanitizedEvent> {
        self.sender.subscribe()
    }
}
