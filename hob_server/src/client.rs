use std::sync::Arc;

use anyhow::{anyhow, Result};
use hob_protocol::{
    decode::Decoder,
    encode::Encoder,
    packet::{
        handshake::{shared_secret, ServerToClientHandshakePacket},
        login::{verify_login, verify_skin},
        network_settings::{CompressionAlgorithmType, NetworkSettingsPacket},
        play_status::PlayStatusPacket,
        resource_pack_info::ResourcePacksInfoPacket,
        resource_pack_response::ResponseStatus,
        resource_pack_stack::ResourcePacksStackPacket,
        PacketKind,
    },
};
use proto_bytes::BytesMut;
use rust_raknet::RaknetSocket;

pub struct Client {
    socket: Arc<RaknetSocket>,
    encoder: Encoder,
    decoder: Decoder,
}

impl Client {
    pub fn new(socket: RaknetSocket) -> Self {
        Client {
            socket: Arc::new(socket),
            encoder: Encoder::default(),
            decoder: Decoder::default(),
        }
    }
    pub async fn listen(&mut self) -> Result<()> {
        loop {
            let buf = self.socket.recv().await.map_err(|e| anyhow!("{:?}", e))?;
            let mut bytes = BytesMut::from(buf.as_slice());
            for packet in self.decoder.decode(&mut bytes)? {
                self.handle(packet).await?;
            }
        }
    }
    pub async fn handle(&mut self, packet: PacketKind) -> Result<()> {
        println!("(CtoS) {}", packet);
        match packet {
            PacketKind::RequestNetworkSetting(v) => {
                let network_setting = NetworkSettingsPacket {
                    compression_threshold: 512,
                    compression_algorithm: CompressionAlgorithmType::Deflate,
                    client_throttle: false,
                    client_throttle_threshold: 0,
                    client_throttle_scalar: 0.0,
                };
                self.send_packet(PacketKind::NetworkSettings(network_setting))
                    .await?;
                self.encoder.force_compress = true;
            }
            PacketKind::Login(v) => {
                let (pubkey, client_data) = verify_login(&v.identity)?;
                let skin = verify_skin(&pubkey, &v.client)?;
                let (ss_key, token) = shared_secret(&pubkey)?;
                self.send_packet(PacketKind::ServerToClientHandshake(
                    ServerToClientHandshakePacket { token },
                ))
                .await?;
                self.encoder.setup_cipher(ss_key);
                self.decoder.setup_cipher(ss_key);
            }
            PacketKind::ClientToServerHandshake(_) => {
                self.send_packet(PacketKind::PlayStatus(PlayStatusPacket::LoginSuccess))
                    .await?;
                let resource_info = ResourcePacksInfoPacket {
                    must_accept: false,
                    has_scripts: false,
                    force_server_packs: false,
                    behaviour_packs: vec![],
                    texture_packs: vec![],
                    resource_pack_links: vec![],
                };
                self.send_packet(PacketKind::ResourcePacksInfo(resource_info))
                    .await?;
            }
            PacketKind::ResourcePackClientResponse(v) => match v.response_status {
                ResponseStatus::HaveAllPacks => {
                    let res_stack = ResourcePacksStackPacket {
                        must_accept: false,
                        behavior_packs: vec![],
                        resource_packs: vec![],
                        game_version: String::from("1.20.50"),
                        experiments: vec![],
                        experiments_previously_used: false,
                    };
                    self.send_packet(PacketKind::ResourcePacksStack(res_stack))
                        .await?;
                }
                ResponseStatus::SendPacks => {}
                ResponseStatus::Completed => {}
                ResponseStatus::Refused | ResponseStatus::None => {
                    self.close().await?;
                }
            },
            _ => {}
        }
        Ok(())
    }
    pub async fn send_packet(&mut self, packet: PacketKind) -> Result<()> {
        println!("(StoC) {}", packet);
        let buffer = self.encoder.encode(packet);
        self.socket
            .send(&buffer, rust_raknet::Reliability::Reliable)
            .await
            .map_err(|e| anyhow!("{:?}", e))?;
        Ok(())
    }
    pub async fn close(&mut self) -> Result<()> {
        self.socket.close().await.map_err(|e| anyhow!("{:?}", e))?;
        println!("disconnect");
        Ok(())
    }
}
