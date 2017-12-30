extern crate reqwest;
extern crate threadpool;

use reqwest::Client;
use serde_json::Value;
use bot::request_sender::{RequestSender, PostParameters};
use bot::updates_receiver::UpdatesReceiver;
use objects::OutgoingMessage;
use objects::OutgoingEdit;
use error::{Result};
use error::Error::{JsonNotFound, RequestFailed};
use objects::{Update};
use std::time::Duration;
use value_extension::ValueExtension;
use std::sync::mpsc::{Receiver};
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
    fn edit_message_text(&self, outgoing_edit: OutgoingEdit);
}

#[derive(Debug)]
pub struct Bot {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: String,
    pub bot_url: String,
    request_sender: RequestSender,
    updates_receiver: UpdatesReceiver,
}

impl TelegramInterface for Bot{
    fn new(bot_token: String) -> Result<Self> {
        let bot_url = [TELEGRAM_BASE_URL, bot_token.as_str()].concat();
        let temp_client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build().unwrap();
        let rjson = Bot::get_me(&temp_client, &bot_url)?;
        let id = rjson.as_required_i64("id")?;
        let first_name = rjson.as_required_string("first_name")?;
        let last_name = rjson.as_optional_string("last_name");
        let username = rjson.as_required_string("username")?;
        Ok(Bot {
            id,
            first_name,
            last_name,
            username,
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

    fn edit_message_text(&self, outgoing_edit: OutgoingEdit){
        let path = "editMessageText";
        let params = outgoing_edit.to_tuple_vec();
        self.post_message(path, params);
    }

}
impl Bot {

    /// API call which gets the information about your bot.
    fn get_me(client: &Client, bot_url: &str) -> Result<Value> {
        let path = "getMe";
        let url = construct_api_url(bot_url, &path);
        let mut resp = client.get(&url).send()?;

        if resp.status().is_success() {
            let rjson: Value = resp.json()?;
            rjson.get("result").cloned().ok_or(JsonNotFound)
        } else {
            Err(RequestFailed(resp.status()))
        }
    }


    /// The actual networking done for sending messages.
    fn post_message(&self, path: &str, params: Vec<(String, String)>){
        let url = construct_api_url(&self.bot_url, path);
        self.request_sender.send(PostParameters {
            path: url.to_string(),
            params
        });
    }
}
