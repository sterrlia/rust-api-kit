use super::{RequestError, RequestMethod, UnexpectedHttpError};

impl<T> From<serde_json::Error> for UnexpectedHttpError<T> {
    fn from(value: serde_json::Error) -> Self {
        UnexpectedHttpError::Request(RequestError::Deserialize)
    }
}

impl<T> From<reqwest::Error> for UnexpectedHttpError<T> {
    fn from(value: reqwest::Error) -> Self {
        tracing::error!("Request error {:?}", value);

        let request_error = if let Some(status) = value.status() {
            RequestError::Http(status.as_u16())
        } else if value.is_timeout() {
            RequestError::Timeout
        } else if value.is_connect() {
            RequestError::Connect
        } else if value.is_redirect() {
            RequestError::Redirect
        } else if value.is_decode() {
            RequestError::Decode
        } else if value.is_builder() {
            RequestError::Builder
        } else {
            RequestError::Unknown
        };

        UnexpectedHttpError::Request(request_error)
    }
}

impl Into<reqwest::Method> for RequestMethod {
    fn into(self) -> reqwest::Method {
        match self {
            RequestMethod::POST => reqwest::Method::POST,
            RequestMethod::GET => reqwest::Method::GET,
            RequestMethod::PUT => reqwest::Method::PUT,
            RequestMethod::DELETE => reqwest::Method::DELETE,
        }
    }
}
