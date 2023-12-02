use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    routing::Router,
};
use shuttle_axum::ShuttleAxum;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR).into_response()
}

async fn day01(Path(nums): Path<String>) -> Response {
    let result = nums
        .split('/')
        .filter_map(|num| num.parse::<isize>().ok())
        .fold(0, |acc, num| acc ^ num)
        .pow(3);
    (StatusCode::OK, result.to_string()).into_response()
}

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
        .route("/1/*nums", get(day01));

    Ok(router.into())
}
