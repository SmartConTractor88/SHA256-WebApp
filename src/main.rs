use actix_files as fs; // for serving static files
use actix_web::{web, App, HttpServer, Responder};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

async fn hash_text(text: String) -> impl Responder {
    let mut hasher = Sha256::new();
    hasher.update(text);
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result); // Convert to hex string
    
    web::Json(HashMap::from([("hash", hash_hex)]))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/hash", web::post().to(hash_text)) // Route for hashing
        .service(fs::Files::new("/static", "./src/static").show_files_listing()) // Serve static files
        .route("/", web::get().to(|| async { fs::NamedFile::open("./src/templates/index.html") })) // Serve the HTML file
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await // Added .await to fix the missing await error
}