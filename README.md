# rust-api-kit

[![crates.io version](https://img.shields.io/crates/v/rust-api-kit.svg)](https://crates.io/crates/rust-api-kit)
[![docs.rs docs](https://docs.rs/rust-api-kit/badge.svg)](https://docs.rs/rust-api-kit)
[![CI build](https://github.com/sterrlia/rust-api-kit/actions/workflows/rust.yml/badge.svg)](https://github.com/sterrlia/rust-api-kit/actions)
[![Dependency Status](https://deps.rs/crate/rust-api-kit/latest/status.svg)](https://deps.rs/crate/rust-api-kit)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.85.1+-ab6000.svg)

## ✨ Features

- Define your API in one place (routes, requests, responses)
- Compile-time checked request/response
- Built for full Rust stacks — server and client
- Async support
- Logging using tracing crate

## 🛠️ Getting Started

### 1. Define Your API in a Shared Crate

- One error response variant for each route
- One shared error response for group of routes

```rust
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: i32,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LoginErrorResponse {
    AccessDenied,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UnexpectedErrorResponse {
    InternalServerError,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthenticatedUnexpectedErrorResponse {
    InternalServerError,
    InvalidAuthToken,
    AuthTokenMissing
}

define_http_routes! {
    group (
        path "api";
        auth BearerToken;
        error AuthenticatedUnexpectedErrorResponse;

        GET "get-messages" GetMessagesRequest => GetMessagesResponse | GetMessagesErrorResponse;
        GET "get-users" GetUsersRequest => GetUsersResponse | GetUsersErrorResponse;
    );

    group (
        path "api";
        error UnexpectedErrorResponse;

        POST "login" LoginRequest => LoginResponse | LoginErrorResponse;
    );
}

```

### 2. Use it in the server (only axum integration available)

- Currently does not check route implementations, only adds route paths defined in macro

```rust
    pub type UnauthenticatedResponse<T, E> = Response<T, E, UnexpectedErrorResponse>;

    pub async fn login(
        extract::State(state): extract::State<state::ServiceState>,
        Json(input): Json<LoginRequest>,
    ) -> UnauthenticatedResponse<LoginResponse, LoginErrorResponse> {
    }

    let http_api_routes = generate_routes! {
        LoginRequest => login,
        GetUsersRequest => get_users,
        GetMessagesRequest => get_messages,
    };

    let app = Router::new()
        .merge(http_api_routes);
```

### 3. Use http client in the frontend app

Response is nested result,
on first level you can match error response shared with other routes
on second level you can match successful response and error response which belongs to that route

```rust
use rust_api_kit::http::client::{HttpClient, BasicHttpClientTrait}

pub fn login(&self, username: String, password: String) -> anyhow::Result<UserData> {
    let request = LoginRequest {
        username,
        password,
    }

    let response = self.http_client.request(request).await??; // LoginResponse

    UserData {
        user_id: response.user_id,
        token: response.token
    }
}
```

### More examples
My messenger project: https://github.com/sterrlia/nultr

## Status

- This project is built for educations purposes
- It is intended for use in my personal projects only, cause I don't think someone would find it useful

## Alternatives

- https://github.com/paperclip-rs/paperclip

## What can be added in future

1. Trait for custom response types
2. Improved logging
3. More auth variants
4. Headers
5. Integrations with backend frameworks
6. Server route implementation checking
7. Websocket client
8. OpenAPI definitions generation
9. Code generation from OpenAPI definitions
10. etc.
