use axum::http::{HeaderMap};

pub async fn header(head:HeaderMap)->String{
    let msg_val = head.get("message").unwrap();
    let msg = msg_val.to_str().unwrap().to_owned();
    msg
}