use std::str::FromStr;

#[derive(Debug)]
pub struct ErrorInfo<T> {
    pub app_code: T,        // HTTP 404 bad request
    pub code: &'static str, // "Not Found"
    pub client_msg: &'static str,
    pub server_msg: String,
}

pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> Result<ErrorInfo<Self::T>, <Self::T as FromStr>::Err>;
}

#[allow(unused)]
impl<T> ErrorInfo<T>
where
    T: FromStr,
{
    pub fn try_new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl std::fmt::Display,
    ) -> Result<Self, T::Err> {
        Ok(Self {
            app_code: T::from_str(app_code)?,
            code,
            client_msg,
            server_msg: server_msg.to_string(),
        })
    }
}
