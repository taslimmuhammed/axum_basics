use serde::{Deserialize, Serialize};
use axum::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson{
    message:String
}
#[derive(Serialize)]
pub struct MirrorResponeseJson{
    message:String,
    name:String,
}
pub async fn mirror_json(Json(body): Json<MirrorJson>)->Json<MirrorResponeseJson>{
    Json(MirrorResponeseJson{
        message:body.message,
        name:String::from("TMM")
    })
}