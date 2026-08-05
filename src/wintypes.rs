#[derive(Clone)]
pub enum WindowType {
    Root,
    Normal,
    Message(String),
    Slowdown,
    Accelerate,
}

impl WindowType {
    pub fn from_id(id: u32) -> Self {
        match id {
            0 => WindowType::Root,
            6 => WindowType::Message("message 6".to_string()),
            17 => WindowType::Message("message 17".to_string()),
            67 => WindowType::Accelerate,
            69 => WindowType::Slowdown,
            _ => WindowType::Normal,
        }
    }
}
