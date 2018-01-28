extern crate reqwest;
extern crate threadpool;
use reqwest::Client;
use std::time::Duration;
use std::thread;
use serde_json;
use serde_json::Value;
use error::check_json_has_ok;
use std::sync::mpsc::{Sender, Receiver, channel};
use objects::Update;
use std::io::Read;
use bot::bot::construct_api_url;
use ::error::Result;
const MAX_UPDATES_PER_REQUEST: i32 = 5;
const SERVER_SIDE_LONG_POLL_TIMEOUT: i32 = 30;

fn construct_get_updates_url_minus_offset(bot_url : &str) -> String{
    let path = "getUpdates";
    let path_url = construct_api_url(bot_url, &path);
    let url = format!("{}?limit={}&timeout={}&allowed_updates=[\"message\",\"callback_query\"]",
                      path_url,
                      MAX_UPDATES_PER_REQUEST,
                      SERVER_SIDE_LONG_POLL_TIMEOUT,
    );
    url
}

#[derive(Debug)]
pub struct UpdatesReceiver{
    bot_url: String,
    updates_sender: Sender<Vec<Update>>,
    updates_receiver: Receiver<Vec<Update>>,
    is_receiving: bool,
}

struct ReceiverThreadData{
    client: Client,
    offset: i64,
    number_errors: u64,
    url_no_offset: String,
    updates_sender: Sender<Vec<Update>>,
}

impl ReceiverThreadData{
    fn errors_backpressure_sleep(&self){
        if self.number_errors > 0 {
            error!("Sleeping for {}", 3*self.number_errors);
            thread::sleep(Duration::from_secs(3*self.number_errors));
        }
    }

    fn get_url_with_offset(&self) -> String{
        format!("{}&offset={}", self.url_no_offset, self.offset)
    }
    fn update_offset(&mut self, updates: &Vec<Update>){
        if let Some(update) = updates.last(){
            self.offset = update.update_id + 1;
        }
    }

    fn get_updates(&self) -> Result<Vec<Update>>{
        let mut data = self.client.get(&self.get_url_with_offset()).send()?;
        let mut response_content = String::new();
        data.read_to_string(&mut response_content)?;
        let json = serde_json::from_str(response_content.as_str())?;
        let json: Value = check_json_has_ok(json)?;
        let updates_json = json.get("result").ok_or(::error::Error::JsonNotFound)?;
        let updates: Vec<Update> = serde_json::from_value(updates_json.clone())?;
        return Ok(updates);
    }

    fn send_updates(&mut self, updates: Vec<Update>){
        if let Err(e) = self.updates_sender.send(updates){
            error!("Could not send update through channel: {}", e);
            self.number_errors += 1;
        }else {
            self.number_errors  = 0;
        }
    }

    fn handle_update(&mut self, updates: Vec<Update>){
        if updates.is_empty(){
            self.number_errors = 0;
        }else {
            self.update_offset(&updates);
            self.send_updates(updates);
        }
    }

    fn handle_update_but_not_send_it(&mut self, updates: &Vec<Update>){
        if updates.is_empty(){
            self.number_errors = 0;
        }else {
            self.update_offset(&updates);
        }
    }


    pub fn discard_pending_updates(&mut self){
        self.offset = -1;
        loop {
            self.errors_backpressure_sleep();
            match self.get_updates(){
                Ok(updates) => {
                    self.handle_update_but_not_send_it(&updates);
                    info!("Cleared previous updates. Last one was: {:?}", updates);
                    return;
                },
                Err(e) => {
                    error!("{:?}", e);
                    self.number_errors += 1;
                }
            }
        }
    }
    fn main_loop(&mut self){
        self.errors_backpressure_sleep();
        match self.get_updates(){
            Ok(updates) => {
                self.handle_update(updates);
            },
            Err(e) => {
                error!("{:?}", e);
                self.number_errors += 1;
            }
        }
    }

}

impl UpdatesReceiver{
    pub fn new(url: String)-> Self{
        let (updates_sender, updates_receiver) = channel();
        UpdatesReceiver{
            bot_url: url,
            updates_sender,
            updates_receiver,
            is_receiving: false,
        }
    }

    pub fn get_updates_channel(&self) -> &Receiver<Vec<Update>>{
        return &self.updates_receiver;
    }




    pub fn start_receiving(&mut self){
        if self.is_receiving{
            error!("Called start_receiving when was already receiving");
            return;
        }
        self.is_receiving = true;
        info!("Starting to receive!");
        let mut receiver_data = ReceiverThreadData{
            client: Client::builder()
                .timeout(Duration::from_secs(40))
                .build().unwrap(),
            offset: -1,
            number_errors: 0,
            url_no_offset: construct_get_updates_url_minus_offset(&self.bot_url),
            updates_sender: self.updates_sender.clone()
        };
        thread::spawn(move ||{
            receiver_data.discard_pending_updates();
            loop {
                receiver_data.main_loop();
            };
        });

    }
}
