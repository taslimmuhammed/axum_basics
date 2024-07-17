use axum::extract::Path;

pub async fn get_path(Path(id):Path<i32>)->String{
    id.to_string()
}