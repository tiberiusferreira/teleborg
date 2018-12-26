extern crate reqwest;
use std::time;
use reqwest::{Client, multipart::Form};
use std::time::Duration;
use crossbeam_channel as channel;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use std::thread;
#[derive(Debug)]
pub struct RequestSender{
    params_sender: Sender<PostParameters>
}

#[derive(Clone, Debug)]
pub struct PostParameters{
    pub path: String,
    pub params: Vec<(String, String)>,
    pub file_to_send: Option<String>
}

impl RequestSender{

    pub fn new() -> RequestSender{
        let (params_sender, params_receiver): (Sender<PostParameters>, Receiver<PostParameters>) = channel::unbounded();
        let request_sender = RequestSender{
            params_sender
        };

        fn create_multipart_form_with_file(params: Vec<(String, String)>, file_path: String) -> Result<Form, ()>{
            let form = reqwest::multipart::Form::new();
            let form_with_params = params.iter().fold(form, |acc_form, &(ref field, ref value)| {
                acc_form.text(field.clone(), value.clone())
            });
            match form_with_params.file("photo", file_path.clone()) {
                Ok(form_with_photo) => return Ok(form_with_photo),
                Err(_) => {
                    error!("Photo not found. Tried path: {}", file_path);
                    return Err(());
                }
            }
        }
        thread::spawn(move ||{
            let client = Client::builder()
                .timeout(Duration::from_secs(10))
                .build().unwrap();
            loop{
                match params_receiver.recv() {
                    Ok(request_params) => {
                        info!("Sending post! {:?}", request_params);
                        let beginning = time::Instant::now();
                        if let Some(file_path) = request_params.file_to_send.clone() {
                            if let Ok(multipart_form)= create_multipart_form_with_file(request_params.clone().params, file_path){
                                let now = time::Instant::now();
                                info!("Took {}s and {}ms to CREATE MULTIPART post",
                                      now.duration_since(beginning).as_secs(),
                                      now.duration_since(beginning).subsec_nanos() as f64/1_000_000.0);
                                match client.post(request_params.path.as_str()).multipart(multipart_form).send() {
                                    Ok(mut resp) => {
                                        info!("Got response: \n{:?}", resp);
                                    },
                                    Err(e) => {
                                        error!("Error sending. {}\n Parameters: {:?}", e, request_params);
                                    }
                                }
                                let now = time::Instant::now();
                                info!("Took {}s and {}ms to send MULTIPART post",
                                      now.duration_since(beginning).as_secs(),
                                      now.duration_since(beginning).subsec_nanos() as f64/1_000_000.0);
                            }
                            continue
                        }
                        match client.post(request_params.path.as_str()).form(&request_params.params).send(){
                            Ok(mut resp) => {
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
                        error!("Error receiving request params. Going to panic. {:?}", e);
                        panic!();
                    }
                }
            }

        });
        return request_sender;
    }
    pub fn send(&self, post_parameters: PostParameters){
        self.params_sender.send(post_parameters);
    }

}
