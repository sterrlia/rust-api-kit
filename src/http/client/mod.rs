pub mod macros;
mod types;
use async_trait::async_trait;
use integration::log_error;
pub use types::*;
pub mod integration;

use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use url::Url;

pub struct HttpClient {
    client: reqwest::Client,
    base_url: Url,
}

impl HttpClient {
    pub fn new(base_url: Url) -> Self {
        let client = reqwest::Client::new();

        Self { client, base_url }
    }
}

impl HttpClientTrait for HttpClient {
    fn get_base_url(&self) -> Url {
        self.base_url.clone()
    }

    fn get_client(&self) -> reqwest::Client {
        self.client.clone()
    }
}

impl BasicHttpClientTrait for HttpClient {}
impl AuthenticatedHttpClientTrait for HttpClient {}

pub trait HttpClientTrait {
    fn get_base_url(&self) -> Url;
    fn get_client(&self) -> reqwest::Client;
}

#[async_trait]
pub trait AuthenticatedHttpClientTrait
where
    Self: HttpClientTrait,
{
    async fn request<R, O, E, U, A>(
        &self,
        request: R,
        auth: A,
    ) -> Result<Result<O, E>, UnexpectedHttpError<U>>
    where
        O: for<'de> Deserialize<'de> + Send + 'static,
        E: for<'de> Deserialize<'de> + Send + std::fmt::Debug + 'static,
        U: for<'de> Deserialize<'de> + Send + std::fmt::Debug + 'static,
        R: HttpRequest<O, E, U> + Serialize + AuthenticatedHttpRequest<A> + Send + 'static,
        A: Auth + Send + 'static,
    {
        let base_url = self.get_base_url();
        let client = self.get_client();

        let mut request_builder = get_request_builder(request, client.clone(), base_url);
        request_builder = auth.add_auth_to_request(request_builder);

        perform::<R, O, E, U>(client, request_builder).await
    }
}

#[async_trait]
pub trait BasicHttpClientTrait
where
    Self: HttpClientTrait,
{
    async fn request<R, O, E, U>(&self, request: R) -> Result<Result<O, E>, UnexpectedHttpError<U>>
    where
        O: for<'de> Deserialize<'de> + Send + 'static,
        E: for<'de> Deserialize<'de> + Send + std::fmt::Debug + 'static,
        U: for<'de> Deserialize<'de> + Send + std::fmt::Debug + 'static,
        R: HttpRequest<O, E, U> + Serialize + Send + 'static,
    {
        let base_url = self.get_base_url();
        let client = self.get_client();
        let request_builder = get_request_builder(request, client.clone(), base_url);
        perform::<R, O, E, U>(client, request_builder).await
    }
}

pub trait Auth {
    fn add_auth_to_request(&self, builder: RequestBuilder) -> RequestBuilder;
}

impl Auth for BearerToken {
    fn add_auth_to_request(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.bearer_auth(self)
    }
}

pub trait AuthenticatedHttpRequest<A> {}

pub trait HttpRequest<O, E, U>
where
    O: for<'de> Deserialize<'de>,
    E: for<'de> Deserialize<'de> + std::fmt::Debug,
    U: for<'de> Deserialize<'de> + std::fmt::Debug,
    Self: Sized + Serialize,
{
    const ENDPOINT: &'static str;
    const METHOD: RequestMethod;

    fn get_url(base_url: Url) -> Url {
        base_url.join(Self::ENDPOINT).unwrap()
    }
}

fn get_request_builder<R, O, E, U>(
    request: R,
    client: reqwest::Client,
    base_url: Url,
) -> RequestBuilder
where
    O: for<'de> Deserialize<'de>,
    E: for<'de> Deserialize<'de> + std::fmt::Debug,
    U: for<'de> Deserialize<'de> + std::fmt::Debug,
    R: HttpRequest<O, E, U>,
{
    let endpoint_url = R::get_url(base_url);
    let method = R::METHOD;

    let mut request_builder = client.request(method.clone().into(), endpoint_url);

    request_builder = match method {
        RequestMethod::GET => request_builder.query(&request),
        _ => request_builder.json(&request),
    };

    request_builder
}

async fn perform<R, O, E, U>(
    client: reqwest::Client,
    request_builder: RequestBuilder,
) -> Result<Result<O, E>, UnexpectedHttpError<U>>
where
    O: for<'de> Deserialize<'de>,
    E: for<'de> Deserialize<'de> + std::fmt::Debug,
    U: for<'de> Deserialize<'de> + std::fmt::Debug,
    R: HttpRequest<O, E, U>,
{
    let request = request_builder.build()?;
    let response = client.execute(request).await?;

    let body_raw = response.text().await?;

    let body: Response<O, E, U> = serde_json::from_str(body_raw.as_str()).inspect_err(|err| {
        log_error(format!(
            "Deserialization error {:?}, {} -> '{}'",
            err,
            std::any::type_name::<R>(),
            body_raw
        ));
    })?;

    match body {
        Response::Ok(ok) => Ok(Ok(ok)),
        Response::Error(error) => {
            log_error(format!(
                "Http error {} -> {:?}",
                std::any::type_name::<R>(),
                error
            ));

            Ok(Err(error))
        }
        Response::UnexpectedError(error) => {
            log_error(format!(
                "Unexpected http error {} -> {:?}",
                std::any::type_name::<R>(),
                error
            ));

            Err(UnexpectedHttpError::Api(error))
        }
    }
}
