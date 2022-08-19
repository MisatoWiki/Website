use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Settings {
    pub misato_api_token: String,
}

impl Settings {
    pub fn init() -> Self {
        dotenv().ok();
        let misato_api_token = match env::var("MISATO_API_TOKEN") {
            Ok(v) => v.to_string(),
            Err(_) => format!(
                "[{}] is not present in the environment!",
                "MISATO_API_TOKEN"
            ),
        };
        Self { misato_api_token }
    }
}
