use axum::{
    http::{header, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use tokio::net::TcpListener;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(idx)) // loading main file for us its index.html 
        .route("/*file", get(stat)); // other paths like css 

        // listening on all 
    let ln = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("ON: http://0.0.0.0:8080");
    axum::serve(ln, app).await.unwrap();
}

async fn idx() -> impl IntoResponse {
    stat("/index.html".parse::<Uri>().unwrap()).await
}

async fn stat(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };
    // search in public, files.
    match Assets::get(path) {
        Some(c) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(header::CONTENT_TYPE, mime.as_ref())], c.data).into_response()
        }
        // if there is no files return 404 
        None => (StatusCode::NOT_FOUND, "404").into_response(),
    }
}