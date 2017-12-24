extern crate reqwest;
extern crate threadpool;
use std::time;
use reqwest::Client;
use std::time::Duration;
use self::threadpool::ThreadPool;
#[derive(Debug)]
pub struct RequestSender{
    client: Client,
    thread_pool: ThreadPool,
}

#[derive(Clone, Debug)]
pub struct PostParameters{
    pub path: String,
    pub params: Vec<(String, String)>
}

impl RequestSender{
    pub fn new() -> RequestSender{
        let request_sender = RequestSender{
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build().unwrap(),
            thread_pool: ThreadPool::new(4),
        };
        return request_sender;
    }
    pub fn send(&self, post_parameters: PostParameters){

        info!("Busy threads: {}", self.thread_pool.active_count());
        let client_clone = self.client.clone();
        let post_parameters_clone = post_parameters.clone();
        self.thread_pool.execute(move ||{
            let beginning = time::Instant::now();
            match client_clone.post(post_parameters_clone.path.as_str()).form(&post_parameters_clone.params).send() {
                Ok(_) => {},
                Err(e) => error!("Error sending. {}\n Parameters: {:?}", e, post_parameters_clone)
            }
            let now = time::Instant::now();
            info!("Took {}s and {}ms to send post",
                  now.duration_since(beginning).as_secs(),
                  now.duration_since(beginning).subsec_nanos() as f64/1_000_000.0)
        });

    }
}
