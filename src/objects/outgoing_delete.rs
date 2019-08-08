#[derive(Clone, Serialize, Debug)]
pub struct OutgoingDelete {
    pub chat_id: i64,
    pub message_id: i64,
}
impl OutgoingDelete {
    pub fn new(chat_id: i64, message_id: i64) -> OutgoingDelete {
        OutgoingDelete {
            chat_id,
            message_id,
        }
    }


    pub fn to_tuple_vec(&self) -> Vec<(String, String)>{
        let as_tuple = vec!(("chat_id".to_string(), self.chat_id.to_string()),
                                ("message_id".to_string(), self.message_id.to_string()));
        as_tuple
    }
}
