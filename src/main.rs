mod apis;
mod data_processing;
mod ml_models;
mod utils;

#[tokio::main]
async fn main() {
    apis::base::config::run_server().await;
}