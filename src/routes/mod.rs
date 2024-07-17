mod hello_world;
mod mirror_json;
mod get_path;
mod query;

use axum::{
    routing::{get, post},
    Router
};
use hello_world::hello_world;
use mirror_json::mirror_json;
use get_path::get_path;
use query::query;

pub fn create_routes()->Router{
    let mut app = Router::new()
            .route("/", get(hello_world));
    app = app.route("/mirror_json", post(mirror_json));
    app = app.route("/path/:id/hello", get(get_path));
    app = app.route("/query", get(query));
    app
}