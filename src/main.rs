mod components;
mod player;
mod protocol;
mod server;

use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new();
    server.launch().await.unwrap();
}
