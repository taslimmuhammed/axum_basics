use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Q{
    name:String,
    id:i32
}
pub async fn query(Query(q):Query<Q>)->Json<Q>{
    Json(q)
}