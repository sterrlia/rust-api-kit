use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::Serialize;

use super::client::Response;

#[macro_export]
macro_rules! generate_routes {
    ( $( $req_ty:ty => $handler:path ),* $(,)? ) => {{
        use axum::{Router, routing::{get, post, put, delete}};

        let mut router = Router::new();
        $(
            router = router.route(
                <$req_ty as $crate::http::client::HttpRequest<_, _, _>>::ENDPOINT,
                match <$req_ty as $crate::http::client::HttpRequest<_, _, _>>::METHOD {
                    $crate::http::client::RequestMethod::GET => get($handler),
                    $crate::http::client::RequestMethod::POST => post($handler),
                    $crate::http::client::RequestMethod::PUT => put($handler),
                    $crate::http::client::RequestMethod::DELETE => delete($handler)
                },
            );
        )*
        router
    }};
}

impl<T, E, U> IntoResponse for Response<T, E, U>
where
    T: Serialize,
    E: Serialize,
    U: Serialize + Clone,
{
    fn into_response(self) -> axum::response::Response {
        let body = Json(self);

        (StatusCode::OK, body).into_response()
    }
}


