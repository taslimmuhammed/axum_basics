use axum::Extension;

use super::middleware_headers::HeaderMessage;


pub async fn middleware_route(Extension(message):Extension<HeaderMessage>)->String{
    message.0
}