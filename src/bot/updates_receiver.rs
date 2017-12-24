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



fn construct_get_updates_url(bot_url : &str) -> String{
    let limit = 5;
    let timeout = 30;
    let path = "getUpdates";
    let path_url = construct_api_url(bot_url, &path);
    let url = format!("{}?limit={}&timeout={}&allowed_updates=[\"message\",\"callback_query\"]",
                      path_url,
                      limit,
                      timeout,
    );
    url
}

#[derive(Debug)]
pub struct UpdatesReceiver{
    client: Client,
    updates_sender: Sender<Vec<Update>>,
    updates_receiver: Receiver<Vec<Update>>,
    url: String,
    is_receiving: bool,
}

impl UpdatesReceiver{
    pub fn new(url: String)-> Self{
        let (updates_sender, updates_receiver) = channel();
        UpdatesReceiver{
            client: Client::builder()
                .timeout(Duration::from_secs(40))
                .build().unwrap(),
            updates_sender,
            updates_receiver,
            url: construct_get_updates_url(&url),
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
        let url = self.url.clone();
        let mut offset = 0;
        let client_clone = self.client.clone();
        let sender_clone = self.updates_sender.clone();
        thread::spawn(move ||{
            loop {
                info!("Sending the request!");
                let url = format!("{}&offset={}", url,offset);
                let mut data = match client_clone.get(&url).send() {
                    Ok(response) => response,
                    Err(e) => {
                        error!("{:?}", e);
                        continue;
                    }
                };
                let mut response_content = String::new();
                if let Err(e) = data.read_to_string(&mut response_content){
                    error!("Could not read response to string: {}", e);
                    continue;
                }
                let json = serde_json::from_str(response_content.as_str());
                let json = match json {
                    Ok(value) => value,
                    Err(e) => {
                        error!("{:?} for response: {}", e, response_content);
                        continue;
                    },
                };
                let json: Value = match check_json_has_ok(json) {
                    Ok(json) => json,
                    Err(e) => {
                        error!("{:?} for response: {}", e, response_content);
                        continue;
                    },
                };
                let updates_json = json.get("result");
                if let Some(result) = updates_json {
                    let updates: Vec<Update> = match serde_json::from_value(result.clone()){
                        Ok(recv_updates) => {
                            recv_updates
                        },
                        Err(e) => {
                            error!("{:?} for response: {}", e, response_content);
                            continue;
                        },
                    };
                    if updates.is_empty() {
                        continue;
                    }
                    offset = (updates.last().unwrap().update_id + 1) as i32;
                    info!("Got updates: {:?}", updates);
                    if let Err(e) = sender_clone.send(updates){
                        error!("Could not send update through channel: {}", e);
                    }
                } else {
                    error!("No key found for response: {}", response_content);
                    continue;
                }
            };
        });

    }
}
