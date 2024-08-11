use error_code_derive::ToErrorInfo;

#[derive(thiserror::Error, Debug, ToErrorInfo)]
#[error_info(app_type = "http::StatusCode", prefix = "api_")]
pub enum MyError {
    #[error("Invalid command: {0}")]
    #[error_info(code = "IC", app_code = "201")]
    InvalidCommand(String),
    #[error("Invalid command: {0}")]
    #[error_info(code = "IA", app_code = "202", client_msg = "friendly msg")]
    InvalidArgument(String),

    #[error("Request timeout")]
    #[error_info(code = "RE", app_code = "500")]
    RespError(#[from] std::io::Error),
}

fn main() {
    let err = MyError::InvalidCommand("cmd".to_string());
    let info = err.to_error_info();
    println!("{:?}", info);
}
