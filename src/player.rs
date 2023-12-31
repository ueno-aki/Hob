use crate::ecs::components::{DeviceOS, PlayerName};
use crate::protocol::mcpe::crypto::cipher::{Aes256CtrManager, Cipher};
use crate::protocol::mcpe::packet::{
    disconnect::DisconnectPacket,
    handshake::{key_exchange, ServerToClientHandshakePacket},
    login::{
        login_verify::{verify_login, verify_skin_data},
        LoginPacket,
    },
    network_settings::{CompressionAlgorithmType, NetworkSettingsPacket},
    play_status::PlayStatusPacket,
    resource_pack_client_response::ResponseStatus,
    resource_pack_stack::ResourcePacksStackPacket,
    resource_packs_info::ResourcePacksInfoPacket,
    PacketKind,
};
use crate::protocol::mcpe::transforms::framer;
use crate::utils::get_option;

use anyhow::{anyhow, Result};
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use rust_raknet::RaknetSocket;
use specs::{Entity, World, WorldExt};
use std::fmt::Display;
use std::sync::Arc;

#[derive(Clone)]
pub struct Player {
    pub entity: Entity,
    world: Arc<AtomicRefCell<World>>,
    pub socket: Arc<RaknetSocket>,
    pub status: Arc<AtomicRefCell<PlayerStatus>>,
}

#[derive(Default)]
pub struct PlayerStatus {
    pub encryption_enabled: bool,
    pub send_counter: u64,
    pub ss_key: Option<[u8; 32]>,
    pub cipher: Option<Cipher>,
    pub decipher: Option<Cipher>,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let world = self.world.borrow();
        let name_storage = world.read_component::<PlayerName>();
        match name_storage.get(self.entity) {
            Some(v) => write!(f, "{}", v.user_name),
            None => write!(f, "{:?}", self.socket.peer_addr()),
        }
    }
}

impl Player {
    pub fn new(socket: RaknetSocket, entity: Entity, world: Arc<AtomicRefCell<World>>) -> Self {
        Player {
            socket: Arc::new(socket),
            world,
            entity,
            status: Arc::new(AtomicRefCell::new(Default::default())),
        }
    }
    #[inline]
    pub fn get_status(&self) -> AtomicRef<PlayerStatus> {
        self.status.try_borrow().unwrap()
    }
    #[inline]
    pub fn get_status_mut(&self) -> AtomicRefMut<PlayerStatus> {
        self.status.try_borrow_mut().unwrap()
    }
    pub async fn listen(&mut self) -> Result<()> {
        if let Err(v) = self.listen_exec().await {
            self.disconnect(format!("ServerError:{:?}", v))
                .await
                .unwrap();
        }
        println!("disconnected,{}", self);
        self.world
            .try_borrow_mut()
            .unwrap()
            .delete_entity(self.entity)
            .unwrap();
        Ok(())
    }
    async fn listen_exec(&mut self) -> Result<()> {
        while let Ok(mut buffer) = self.socket.recv().await {
            let buffer: &mut [u8] = buffer[1..].as_mut();
            self.decrypt_or(buffer);
            for pkt in framer::decode(buffer)? {
                let packet = PacketKind::from_buf(&pkt, 0)?;
                println!("[C=>S]{}", packet);
                self.handle_packet(&packet).await?
            }
        }
        Ok(())
    }
    async fn handle_packet(&mut self, packet: &PacketKind) -> Result<()> {
        match packet {
            PacketKind::RequestNetworkSettingPacket(pkt) => {
                let current_p: i32 = get_option("protocol").unwrap().parse().unwrap();
                match pkt.client_protocol {
                    x if x > current_p => self.send_packet(PlayStatusPacket::FailedSpawn).await?,
                    x if x < current_p => self.send_packet(PlayStatusPacket::FailedClient).await?,
                    _ => self.send_network_setting().await?,
                };
            }
            PacketKind::LoginPacket(pkt) => self.login_handshake(pkt).await?,
            PacketKind::ClientToServerHandshakePacket(_) => {
                self.send_packet(PlayStatusPacket::LoginSuccess).await?;
                let resource_info = ResourcePacksInfoPacket {
                    must_accept: false,
                    scripting: false,
                    force_server_packs: false,
                    behaviour_pack_infos: vec![],
                    resource_pack_infos: vec![],
                    resource_pack_links: vec![],
                };
                self.send_packet(resource_info).await?;
            }
            PacketKind::ClientCacheStatusPacket(_) => {}
            PacketKind::ResourcePackClientResponsePacket(v) => match v.response_status {
                ResponseStatus::HaveAllPacks => {
                    let res_stack = ResourcePacksStackPacket {
                        must_accept: false,
                        behavior_packs: vec![],
                        resource_packs: vec![],
                        game_version: "1.20.30".to_owned(),
                        experiments: vec![],
                        is_experimental: false,
                    };
                    self.send_packet(res_stack).await?;
                }
                _ => {}
            },
            _ => todo!(),
        }
        Ok(())
    }
    pub async fn send_packet<T: Into<PacketKind>>(&mut self, packet: T) -> Result<()> {
        let packet: PacketKind = packet.into();
        let mut buffer = framer::encode(&packet, self.get_status().encryption_enabled)?;
        self.encrypt_or(&mut buffer)?;
        self.socket
            .send(
                &[vec![0xfe], buffer].concat(),
                rust_raknet::Reliability::Reliable,
            )
            .await
            .map_err(|e| anyhow!("FailedToSendPacket:{:?}", e))?;
        println!("[S=>C]{}", packet);
        Ok(())
    }
    async fn send_network_setting(&mut self) -> Result<()> {
        let network_setting = NetworkSettingsPacket {
            compression_threshold: 512,
            compression_algorithm: CompressionAlgorithmType::Deflate,
            client_throttle: false,
            client_throttle_threshold: 0,
            client_throttle_scalar: 0.0,
        };
        self.send_packet(network_setting).await?;
        Ok(())
    }
    async fn disconnect(&mut self, message: String) -> Result<()> {
        let disconnect = DisconnectPacket {
            message,
            hide_disconnect_reason: false,
        };
        self.send_packet(disconnect).await?;
        Ok(())
    }
    async fn login_handshake(&mut self, login: &LoginPacket) -> Result<()> {
        let (key, user_data) = verify_login(&login.identity)?;
        let skin_data = verify_skin_data(&key, &login.client)?;

        let (secret, token) = key_exchange::shared_secret(&key)?;
        let iv: [u8; 16] = [secret[0..12].to_vec(), vec![0, 0, 0, 2]]
            .concat()
            .try_into()
            .unwrap();

        self.send_packet(ServerToClientHandshakePacket { token })
            .await?;

        self.get_status_mut().encryption_enabled = true;
        self.get_status_mut().ss_key = Some(secret.clone());
        self.setup_cipher(&secret, &iv)?;

        let world = self.world.try_borrow().unwrap();
        let mut os_storage = world.write_component::<DeviceOS>();
        os_storage.insert(self.entity, DeviceOS::from(skin_data.DeviceOS))?;

        let player_name = PlayerName {
            xuid: user_data.XUID,
            client_uuid: user_data.identity,
            user_name: user_data.displayName,
        };
        let mut name_storage = world.write_component::<PlayerName>();
        name_storage.insert(self.entity, player_name)?;
        Ok(())
    }
}
