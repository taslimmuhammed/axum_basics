use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let message = req
        .headers()
        .get("message")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_owned();

    req.extensions_mut().insert(HeaderMessage(message));

    Ok(next.run(req).await)
}