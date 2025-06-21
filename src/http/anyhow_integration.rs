use super::client::Response;

impl<T, E, U> From<anyhow::Error> for Response<T, E, U>
where
    U: From<anyhow::Error>,
{
    fn from(value: anyhow::Error) -> Self {
        tracing::error!("Unexpected error: {:?}", value);

        Response::UnexpectedError(value.into())
    }
}

