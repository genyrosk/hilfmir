#[derive(Debug)]
pub struct AppError {
    pub msg: String,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

impl std::error::Error for AppError {}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError {
            msg: format!("reqwest::Error: {}", e.to_string()),
        }
    }
}

impl From<teloxide::RequestError> for AppError {
    fn from(e: teloxide::RequestError) -> Self {
        AppError {
            msg: format!("teloxide::RequestError: {}", e.to_string()),
        }
    }
}
