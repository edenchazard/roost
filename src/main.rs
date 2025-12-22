use axum::Router;
use roost::{controllers::index_controller, directory_reader};

#[tokio::main]
async fn main() {
    let app = Router::new().merge(index_controller());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    let input_dir: &str = "/root/roost/in";

    match directory_reader::list_files_in_dir(input_dir) {
        Ok((supported_files, unsupported_paths)) => {
            println!("Supported files:");
            for track in &supported_files {
                println!("{:?}", track);
            }
            println!("Unsupported files:");
            for path in &unsupported_paths {
                println!("{}", path);
            }
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return;
        }
    };
}
