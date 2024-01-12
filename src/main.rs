use hob_server::Server;

#[tokio::main]
async fn main() {
    let server = Server;
    println!("Server Started");
    server.listen().await;
}
