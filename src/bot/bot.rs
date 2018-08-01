extern crate reqwest;
extern crate threadpool;

use reqwest::Client;
use serde_json::Value;
use bot::request_sender::{RequestSender, PostParameters};
use bot::updates_receiver::UpdatesReceiver;
use objects::OutgoingMessage;
use objects::OutgoingPhoto;
use objects::OutgoingChannelMessage;
use objects::AnswerCallbackQuery;
use objects::OutgoingEdit;
use error::{Result};
use error::Error::{JsonNotFound, RequestFailed};
use objects::{Update};
use std::time::Duration;
use value_extension::ValueExtension;
use crossbeam_channel::Receiver;
use std;
/// A `Bot` which will do all the API calls.
const TELEGRAM_BASE_URL: &'static str = "https://api.telegram.org/bot";
pub fn construct_api_url(bot_url: &str, path: &str) -> String {
    format!("{}/{}", bot_url, path)
}

pub trait TelegramInterface {
    fn new(bot_token: String) -> Result<Self> where Self: std::marker::Sized;
    fn start_getting_updates(&mut self);
    fn get_updates_channel(&self) -> &Receiver<Vec<Update>>;
    fn send_msg(&self, outgoing_message: OutgoingMessage);
    fn send_channel_msg(&self, outgoing_message: OutgoingChannelMessage);
    fn edit_message_text(&self, outgoing_edit: OutgoingEdit);
    fn send_callback_answer(&self, callback_answer: AnswerCallbackQuery);
    fn send_photo(&self, outgoing_photo: OutgoingPhoto);
    }

#[derive(Debug)]
pub struct Bot {
    pub bot_user_info: BotUserInfo,
    pub bot_url: String,
    request_sender: RequestSender,
    updates_receiver: UpdatesReceiver,
}

#[derive(Debug)]
pub struct BotUserInfo{
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
}

impl TelegramInterface for Bot{
    fn new(bot_token: String) -> Result<Self> {
        let bot_url = [TELEGRAM_BASE_URL, bot_token.as_str()].concat();
        let temp_client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build().unwrap();
        let bot_user_info = Bot::get_me(&temp_client, &bot_url);
        Ok(Bot {
            bot_user_info,
            bot_url: bot_url.clone(),
            request_sender: RequestSender::new(),
            updates_receiver: UpdatesReceiver::new(bot_url),
        })
    }

    fn start_getting_updates(&mut self){
        info!("Asking for bot updates!");
        self.updates_receiver.start_receiving();
    }

    fn get_updates_channel(&self) -> &Receiver<Vec<Update>>{
        self.updates_receiver.get_updates_channel()
    }

    fn send_msg(&self, outgoing_message: OutgoingMessage){
        let path = "sendMessage";
        let params = outgoing_message.to_tuple_vec();
        self.post_message(path, params)
    }

    fn send_channel_msg(&self, outgoing_channel_message: OutgoingChannelMessage){
        let path = "sendMessage";
        let params = outgoing_channel_message.to_tuple_vec();
        self.post_message(path, params)
    }

    fn edit_message_text(&self, outgoing_edit: OutgoingEdit){
        let path = "editMessageText";
        let params = outgoing_edit.to_tuple_vec();
        self.post_message(path, params);
    }

    fn send_callback_answer(&self, callback_answer: AnswerCallbackQuery){
        let path = "answerCallbackQuery";
        let params = callback_answer.to_tuple_vec();
        self.post_message(path, params)
    }

    fn send_photo(&self, outgoing_photo: OutgoingPhoto){
        let path = "sendPhoto";
//        let params = outgoing_photo.to_tuple_vec();
        self.post_photo(path, outgoing_photo)
    }

}
impl Bot {

    fn get_me(client: &Client, bot_url: &str) -> BotUserInfo{
        let mut number_errors_up_to_2000 = 0;
        loop {
            match Bot::try_get_me(&client, &bot_url) {
                Ok(bot_user_info) => return bot_user_info,
                Err(e) => {
                    number_errors_up_to_2000 = (number_errors_up_to_2000 + 1) % 2000;
                    error!("Error getting bot info: {:?}", e);
                    error!("Sleeping for: {} seconds.", 60*number_errors_up_to_2000);
                    std::thread::sleep(Duration::from_secs(60*number_errors_up_to_2000));
                }
            }
        }

    }
    /// API call which gets the information about your bot.
    fn try_get_me(client: &Client, bot_url: &str) -> Result<BotUserInfo> {
        let path = "getMe";
        let url = construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send()?;
        if resp.status().is_success() {
            let rjson: Value = resp.json()?;
            let rjson = rjson.get("result").ok_or(JsonNotFound)?;
            let id = rjson.as_required_i64("id")?;
            let first_name = rjson.as_required_string("first_name")?;
            let last_name = rjson.as_optional_string("last_name");
            let username = rjson.as_required_string("username")?;
            return Ok(BotUserInfo{
                id,
                first_name,
                last_name,
                username,
            })
        } else {
            Err(RequestFailed(resp.status()))
        }
    }

    fn post_photo(&self, path: &str, outgoing_photo: OutgoingPhoto){
        let url = construct_api_url(&self.bot_url, path);

        self.request_sender.send(PostParameters {
            path: url.to_string(),
            params: outgoing_photo.to_tuple_vec(),
            file_to_send: Some(outgoing_photo.photo_path)
        });
    }

    /// The actual networking done for sending messages.
    fn post_message(&self, path: &str, params: Vec<(String, String)>){
        let url = construct_api_url(&self.bot_url, path);
        self.request_sender.send(PostParameters {
            path: url.to_string(),
            params,
            file_to_send: None
        });
    }
}
