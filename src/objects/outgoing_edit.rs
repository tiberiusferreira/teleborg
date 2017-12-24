use objects::parse_mode::{ParseMode, get_parse_mode};
use objects::{InlineKeyboardMarkup};
use serde_json;
#[derive(Clone, Serialize, Debug)]
pub struct OutgoingEdit {
    pub chat_id: i64,
    pub message_id: i64,
    pub text: String,
    pub parse_mode: ParseMode,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl OutgoingEdit{
    pub fn new(chat_id: i64, message_id: i64, text: &str) -> OutgoingEdit{
        OutgoingEdit{
            chat_id,
            message_id,
            text: text.to_string(),
            parse_mode: ParseMode::Markdown,
            reply_markup: None
        }
    }

    pub fn with_reply_markup(&mut self, markup: InlineKeyboardMarkup){
        self.reply_markup = Some(markup);
    }

    pub fn to_tuple_vec(&self) -> Vec<(String, String)>{
        let mut as_tuple = vec!(("chat_id".to_string(), self.chat_id.to_string()),
                                ("message_id".to_string(), self.message_id.to_string()),
                                ("text".to_string(), self.text.to_string()),
                                ("parse_mode".to_string(), get_parse_mode(&self.parse_mode).to_string()));
        if let Some(ref reply_markup) = self.reply_markup{
            let reply_markup =
                serde_json::to_string(&reply_markup).unwrap_or("".to_string());
            as_tuple.push(("reply_markup".to_string(), reply_markup.to_string()));
        }
        as_tuple
    }
}
