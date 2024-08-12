use std::{
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

pub struct ErrorInfo<T> {
    pub app_code: T,        // HTTP 404 bad request
    pub code: &'static str, // "ISE"
    pub hash: String,
    pub client_msg: &'static str,
    pub server_msg: String,
}

pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> ErrorInfo<Self::T>;
}

#[allow(unused)]
impl<T> ErrorInfo<T>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    pub fn new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl std::fmt::Display,
    ) -> Self {
        let server_msg = server_msg.to_string();

        let mut hasher = DefaultHasher::default();
        server_msg.hash(&mut hasher);
        let hash = hasher.finish();
        let hash = URL_SAFE_NO_PAD.encode(hash.to_be_bytes());
        Self {
            app_code: T::from_str(app_code).expect("cannot convert app_code"),
            code,
            hash,
            client_msg,
            server_msg,
        }
    }
}

impl<T> ErrorInfo<T> {
    pub fn client_msg(&self) -> &str {
        if self.client_msg.is_empty() {
            self.server_msg.as_ref()
        } else {
            self.client_msg
        }
    }
}

// Display: for client facing error messages
impl<T> fmt::Display for ErrorInfo<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.client_msg())
    }
}

// Debug: for server log
impl<T> fmt::Debug for ErrorInfo<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.server_msg)
    }
}
