use objects::{User, Chat};

/// Represents a Telegram message.
#[derive(Clone, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub date: i64,
    pub chat: Chat,
    pub text: Option<String>
}
