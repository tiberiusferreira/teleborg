extern crate reqwest;
use std::time;
use reqwest::Client;
use std::time::Duration;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
#[derive(Debug)]
pub struct RequestSender{
    params_sender: Sender<PostParameters>
}

#[derive(Clone, Debug)]
pub struct PostParameters{
    pub path: String,
    pub params: Vec<(String, String)>
}

impl RequestSender{

    pub fn new() -> RequestSender{
        let (params_sender, params_receiver): (Sender<PostParameters>, Receiver<PostParameters>) = channel();
        let request_sender = RequestSender{
            params_sender
        };
        thread::spawn(move ||{
            let client = Client::builder()
                .timeout(Duration::from_secs(6))
                .build().unwrap();
            loop{
                match params_receiver.recv() {
                    Ok(request_params) => {
                        info!("Sending post! {:?}", request_params);
                        let beginning = time::Instant::now();
                        match client.post(request_params.path.as_str()).form(&request_params.params).send(){
                            Ok(mut resp) => {
//                                if let Err(e) = resp.copy_to(&mut ::std::io::sink()){
//                                    error!("Error copying response to iosink. {}", e);
//                                }
                                info!("Got response: \n{:?}", resp);
                            },
                            Err(e) => {
                                error!("Error sending. {}\n Parameters: {:?}", e, request_params);
                            }
                        }
                        let now = time::Instant::now();
                        info!("Took {}s and {}ms to send post",
                              now.duration_since(beginning).as_secs(),
                              now.duration_since(beginning).subsec_nanos() as f64/1_000_000.0);
                    },
                    Err(e) => {
                        error!("Error receiving request params. Going to panic : {}", e);
                        panic!();
                    }
                }
            }

        });
        return request_sender;
    }
    pub fn send(&self, post_parameters: PostParameters){
        self.params_sender.send(post_parameters).unwrap();
    }
}
