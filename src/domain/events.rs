use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ChannelDeactivatedEvent {
    pub channel_id: String,
    pub timestamp: String,
}
