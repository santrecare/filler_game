pub mod game;
pub mod server;


#[tokio::main]
async fn main() {
    server::run_server::run_server().await;
}
