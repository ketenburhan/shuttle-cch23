use axum::{routing::get, Router, response::{Response, IntoResponse}, http::StatusCode};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error));

    Ok(router.into())
}
