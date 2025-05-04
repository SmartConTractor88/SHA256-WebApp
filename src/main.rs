use actix_files as fs; // for serving static files
use actix_web::{web, App, HttpServer, Responder}; // actix-web components
use std::collections::HashMap; // HashMap for storing key-value pairs
use sha2::{Sha256, Digest}; // hashing functionality

// asynchronous function to hash input text and return the result as JSON
async fn hash_text(text: String) -> impl Responder {
    let mut hasher = Sha256::new(); // create a new SHA256 hasher
    hasher.update(text); // update the hasher with the input text
    let result = hasher.finalize(); // finalize the hash computation
    let hash_hex = format!("{:x}", result); // convert the hash to a hexadecimal string
    
    web::Json(HashMap::from([("hash", hash_hex)])) // return the hash as a JSON response
}

#[actix_web::main] // mark the main function as the Actix runtime entry point
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { // create a new HTTP server
        App::new() // create a new Actix application
        .route("/hash", web::post().to(hash_text)) // define the /hash route for POST requests
        .service(fs::Files::new("/static", "./src/static").show_files_listing()) // serve static files from the /static directory
        .route("/", web::get().to(|| async { fs::NamedFile::open("./src/templates/index.html") })) // serve the index.html file for the root route
    })
    .bind("127.0.0.1:8080")? // bind the server to localhost
    .run() // run the server
    .await // await the server's completion
}