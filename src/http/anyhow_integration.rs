use super::client::{integration::log_error, Response};

impl<T, E, U> From<anyhow::Error> for Response<T, E, U>
where
    U: From<anyhow::Error>,
{
    fn from(value: anyhow::Error) -> Self {
        log_error(format!("Unexpected error: {:?}", value));

        Response::UnexpectedError(value.into())
    }
}
