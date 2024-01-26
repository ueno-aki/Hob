mod client;

use rust_raknet::RaknetListener;

use crate::client::Client;

pub struct Server;

impl Server {
    pub async fn listen(&self) {
        let mut listener = RaknetListener::bind(&"0.0.0.0:19132".parse().unwrap())
            .await
            .unwrap();
        listener.listen().await;
        while let Ok(socket) = listener.accept().await {
            tokio::spawn(async move {
                let mut client = Client::new(socket);
                if let Err(e) = client.listen().await {
                    println!("{:?}", e);
                }
            });
        }
    }
}
