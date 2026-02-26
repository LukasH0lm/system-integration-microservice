use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelName(String);

impl ChannelName {
    pub fn new(name: String) -> Result<Self, &'static str> {
        if name.len() < 3 || name.len() > 20 {
            return Err("Channel name must be between 3 and 20 characters");
        }
        if name.contains(" ") {
            return Err("Channel name cannot contain spaces");
        }
        Ok(ChannelName(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Added Clone so we can store copies in our in-memory vector
#[derive(Debug, Clone)]
pub struct Channel {
    id: Uuid,
    name: ChannelName,
    is_active: bool,
}

impl Channel {
    pub fn new(name: ChannelName) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            is_active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    // Added the rename method for your PUT endpoint
    pub fn rename(&mut self, new_name: ChannelName) {
        self.name = new_name;
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &ChannelName {
        &self.name
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
