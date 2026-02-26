use crate::domain::channel::Channel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelShared {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
}

impl From<&Channel> for ChannelShared {
    fn from(channel: &Channel) -> Self {
        Self {
            id: channel.id(),
            name: channel.name().as_str().to_string(),
            is_active: channel.is_active(),
        }
    }
}
