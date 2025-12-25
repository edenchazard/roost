use axum::Router;
use roost::controllers::index_controller;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(index_controller());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
