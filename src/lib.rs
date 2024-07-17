mod routes;

pub async fn run(){
    let app = routes::create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at localhost:3000");
    axum::serve(listener, app).await.unwrap();
}