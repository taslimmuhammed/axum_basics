mod hello_world;
mod mirror_json;
mod get_path;
mod query;
mod header;
mod middleware_headers;
mod middleware_route;
mod returns_201;
mod return_json;
use axum::{
    http::Method, middleware, routing::{get, post}, Router
};
use tower_http::cors::{Any, CorsLayer};
use hello_world::hello_world;
use mirror_json::mirror_json;
use get_path::get_path;
use query::query;
use header::header;
use middleware_headers::auth;
use middleware_route::middleware_route;
use returns_201::returns_201;
use return_json::return_json;
pub fn create_routes()->Router{
    let cors  = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    let app = Router::new()
        .route("/middleware", get(middleware_route))
        .layer(middleware::from_fn(auth))
        .route("/", get(hello_world))
        .route("/mirror_json", post(mirror_json))
        .route("/path/:id/hello", get(get_path))
        .route("/query", get(query))
        .route("/header", get(header))
        .route("/returns_201", get(returns_201))
        .route("/returns_json", get(return_json))
        .layer(cors);
    app
}