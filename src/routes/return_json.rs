use axum::Json;
use serde::Serialize;


#[derive(Serialize)]
pub struct ModalJson{
    name: String,
    id: i32
}
pub async fn return_json()-> Json<ModalJson>{
    Json(ModalJson{
        name:"TMM".to_owned(),
        id:67
    })
}