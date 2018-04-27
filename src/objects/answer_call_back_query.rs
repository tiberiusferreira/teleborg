
#[derive(Clone, Deserialize, Debug)]
pub struct AnswerCallbackQuery {
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
}

impl AnswerCallbackQuery{
    pub fn to_tuple_vec(&self) -> Vec<(String, String)>{
        let mut as_tuple = vec!(("callback_query_id".to_string(), self.callback_query_id.clone()));
        if let Some(ref text) = self.text{
            as_tuple.push(("text".to_string(), text.clone()));
        }
        if let Some(ref show_alert) = self.show_alert{
            as_tuple.push(("show_alert".to_string(), show_alert.to_string()));
        }
        as_tuple
    }
}