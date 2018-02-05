use objects::parse_mode::{ParseMode, get_parse_mode};
use objects::{InlineKeyboardMarkup};
use serde_json;
/// Represents a Telegram message.
#[derive(Clone, Serialize, Debug)]
pub struct OutgoingChannelMessage {
    pub channel_name: String,
    pub text: String,
    pub parse_mode: ParseMode,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl OutgoingChannelMessage{
    pub fn new(channel_name: String, text: &str) -> OutgoingChannelMessage{
        OutgoingChannelMessage{
            channel_name,
            text: text.to_string(),
            parse_mode: ParseMode::Markdown,
            reply_to_message_id: None,
            reply_markup: None
        }
    }

    pub fn with_reply_msg_id(&mut self, reply_to_message_id: i64){
        self.reply_to_message_id = Some(reply_to_message_id);
    }


    pub fn with_reply_markup(&mut self, markup: Vec<Vec<String>>){
        self.reply_markup = Some(InlineKeyboardMarkup::new(markup));
    }

    pub fn to_tuple_vec(&self) -> Vec<(String, String)>{
        let mut as_tuple = vec!(("chat_id".to_string(), self.channel_name.clone()),
             ("text".to_string(), self.text.to_string()),
             ("parse_mode".to_string(), get_parse_mode(&self.parse_mode).to_string()));
        if let Some(ref reply_to_message_id) = self.reply_to_message_id{
            as_tuple.push(("reply_to_message_id".to_string(), reply_to_message_id.to_string()));
        }
        if let Some(ref reply_markup) = self.reply_markup{
            let reply_markup =
                serde_json::to_string(&reply_markup).unwrap_or("".to_string());
            as_tuple.push(("reply_markup".to_string(), reply_markup.to_string()));
        }
        as_tuple
    }
}
