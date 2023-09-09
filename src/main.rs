mod components;
mod player;
mod protocol;
mod server;
#[cfg(test)]
mod tests;

use server::Server;

#[tokio::main]
async fn main() {
    let mut server = Server::new();
    server.launch().await.unwrap();
}
