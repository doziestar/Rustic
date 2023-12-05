use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, prelude::*, registry::Registry};
use crate::apis::domains::status::status_routes;

/// run_server is a function that starts the server
pub async fn run_server() {
    // Set up the tracing
    // let subscriber = Registry::default()
    //     .with(fmt::layer())
    //     .with(TraceLayer::new_for_http());

    // Set the subscriber as the global default
    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("Unable to set global default subscriber");

    // Create your application with the necessary routes
    let app = Router::new()
        .nest("/status", status_routes())
        .layer(TraceLayer::new_for_http());

    // Create the address and the TcpListener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Unable to bind to address");

    println!("Summoning server spirits on {:?}", listener.local_addr().unwrap());

    // Serve the app using the listener
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
