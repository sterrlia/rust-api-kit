use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum RequestMethod {
    POST,
    GET,
    PUT,
    DELETE,
}

#[derive(Clone, Debug)]
pub enum UnexpectedHttpError<E> {
    Request(RequestError),
    Api(E),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Response<T, E, U> {
    Ok(T),
    Error(E),
    UnexpectedError(U),
}

pub type BearerToken = String;

#[derive(Clone, Debug)]
pub enum RequestError {
    Deserialize,
    Builder,
    Http(u16),
    Timeout,
    Connect,
    Redirect,
    Unknown,
    Decode,
}
