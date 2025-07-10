use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir; 

mod models;      
mod transit_api; 
mod web; 
use crate::web::handlers;


#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); 

    let api_key = env::var("API_KEY")
        .expect("API_KEY must be set in the .env file or environment");

    println!("API Key loaded successfully."); // Don't print the actual key!

    let app = Router::new()
        .route("/", get(handlers::root_handler))
        .route("/api/routes", get(handlers::root_handler))
        .nest_service("/static", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // Listen on localhost, port 3000
    println!("listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service())
        .await?;

    Ok(())
}