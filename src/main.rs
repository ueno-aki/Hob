mod components;
mod player;
mod server;

use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new();
    server.launch().await.unwrap();
}
