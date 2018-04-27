use objects::User;
use objects::Message;
use objects::CallBackQuery;
use objects::Update;


#[derive(Clone, Debug)]
pub enum CleanedUpdate{
    CleanedMessage(CleanedMessage),
    CleanedCallbackQuery(CleanedCallbackQuery)
}

pub fn clean_update(update: Update) -> Result<CleanedUpdate, UpdateError> {
    if let Some(message) = update.message{
        return Ok(CleanedUpdate::CleanedMessage(CleanedMessage::from_message(message)?));
    }
    if let Some(callback_query) = update.callback_query{
        return Ok(CleanedUpdate::CleanedCallbackQuery(CleanedCallbackQuery::new(callback_query)?));
    }
    return Err(UpdateError::NoMsgOrCallback);
}

#[derive(Debug, Fail)]
pub enum UpdateError {
    #[fail(display = "Message had no text")]
    MsgWithoutText,
    #[fail(display = "Message with no sender")]
    MsgWithoutSender,
    #[fail(display = "Callback without original message")]
    CallbackWithoutOriginalMsg,
    #[fail(display = "Callback without data")]
    CallbackWithoutData,
    #[fail(display = "Neither callback or message")]
    NoMsgOrCallback
}

#[derive(Clone, Debug)]
pub struct CleanedMessage {
    pub sender_id: i64,
    pub chat_id: i64,
    pub text: String
}

fn get_sender_id(maybe_sender: Option<User>) -> Result<i64, UpdateError>{
    match maybe_sender{
        Some(sender) => return Ok(sender.id),
        None => {
            return Err(UpdateError::MsgWithoutSender);
        }
    };
}

impl CleanedMessage {
    pub fn from_message(message: Message) -> Result<Self, UpdateError>{
        let chat_id = message.chat.id;
        let text = match message.text{
            Some(txt) => txt,
            None => {
                return Err(UpdateError::MsgWithoutText);
            }
        };
        let sender_id = get_sender_id(message.from)?;
        Ok(CleanedMessage {
            sender_id,
            chat_id,
            text
        })
    }
}

#[derive(Clone, Debug)]
pub struct CleanedCallbackQuery{
    pub sender_id: i64,
    pub callback_id: String,
    pub original_msg_id: i64,
    pub original_msg_chat_id: i64,
    pub data: String
}

impl CleanedCallbackQuery{
    pub fn new(callback: CallBackQuery) -> Result<Self, UpdateError>{
        let original_message = match callback.message {
            Some(message) => message,
            None => {
                return Err(UpdateError::CallbackWithoutOriginalMsg);
            }
        };
        let sender_id = callback.from.id;
        let data = match callback.data{
            Some(data) => data,
            None => {
                return Err(UpdateError::CallbackWithoutData);
            }
        };
        Ok(CleanedCallbackQuery{
            sender_id,
            callback_id: callback.id,
            original_msg_id: original_message.message_id,
            original_msg_chat_id: original_message.chat.id,
            data,
        })
    }
}