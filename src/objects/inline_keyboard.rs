use std::option::Option;

/// Represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct InlineKeyboardButton {
    text: String,
    callback_data: Option<String>,
}

impl InlineKeyboardMarkup {
    pub fn new(buttons: Vec<Vec<String>>) -> InlineKeyboardMarkup {
        let vec_buttons = buttons.iter().map(|button_vec| {
            button_vec.iter().map(|button|{
                InlineKeyboardButton {
                    text: button.to_string(),
                    callback_data: Some(button.to_string()),
                }
            }).collect()
        }).collect();
        InlineKeyboardMarkup { inline_keyboard: vec_buttons }
    }

}

impl InlineKeyboardButton {
    pub fn new(text: String,
               callback_data: Option<String>)
               -> InlineKeyboardButton {
        InlineKeyboardButton {
            text,
            callback_data: Some(callback_data.unwrap_or("".to_string())),
        }
    }
}
