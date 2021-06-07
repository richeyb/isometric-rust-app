use serde::{Serialize, Deserialize};

pub fn get_data() -> String {
    "Hello World".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataResponse {
    pub status: String,
}