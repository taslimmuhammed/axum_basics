mod hello_world;
mod mirror_json;
mod get_path;
mod query;
mod header;

use axum::{
    http::Method, routing::{get, post}, Router
};
use tower_http::cors::{Any, CorsLayer};
use hello_world::hello_world;
use mirror_json::mirror_json;
use get_path::get_path;
use query::query;
use header::header;

pub fn create_routes()->Router{
    let cors  = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    let mut app = Router::new();
    app = app.route("/", get(hello_world));
    app = app.route("/mirror_json", post(mirror_json));
    app = app.route("/path/:id/hello", get(get_path));
    app = app.route("/query", get(query));
    app = app.route("/header", get(header));
    app = app.layer(cors);
    app
}