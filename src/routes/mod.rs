mod hello_world;
mod mirror_json;

use axum::{
    routing::{get, post},
    Router
};
use hello_world::hello_world;
use mirror_json::mirror_json;

pub fn create_routes()->Router{
    Router::new()
            .route("/", get(hello_world))
            .route("/mirror_json", post(mirror_json))
}