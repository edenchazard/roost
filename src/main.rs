use axum::Router;
use roost::controllers::index_controller;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(index_controller());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[test]
fn clean_dirs() {
    std::fs::remove_dir_all("assets/album-art").ok();
}

#[test]
fn ensure_dirs_exist() {
    let directories: Vec<&str> = vec!["assets/album-art", "assets/media"];

    for dir in directories {
        match std::fs::create_dir_all(dir) {
            Ok(_) => println!("Directory '{}' ensured.", dir),
            Err(e) => eprintln!("Error creating directory '{}': {}", dir, e),
        }
    }
}
