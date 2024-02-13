use anyhow::{bail, Error, Result};
use hob_protocol::packet::{
    disconnect::DisconnectPacket,
    handshake::{shared_secret, ServerToClientHandshakePacket},
    login::{verify_login, verify_skin, ExtraUserdata, LoginPacket, SkinData},
    network_settings::{CompressionAlgorithmType, NetworkSettingsPacket},
    play_status::PlayStatusPacket,
    PacketKind,
};

use crate::connection_client::ConnectionClient;

#[derive(Debug)]
pub enum LoginResult {
    Success(Box<SkinData>, ExtraUserdata),
    Failed(Error),
}

static PROTOCOL_VERSION: i32 = 649;

pub async fn login_process(connection: &mut ConnectionClient) -> Result<LoginResult> {
    if let Err(e) = handle_request(connection).await {
        return Ok(LoginResult::Failed(e));
    }

    match handle_login(connection).await {
        Ok((skin, userdata)) => Ok(LoginResult::Success(skin, userdata)),
        Err(e) => {
            connection
                .write(DisconnectPacket::from("disconnectionScreen.notAuthenticated").into())
                .await?;
            Ok(LoginResult::Failed(e))
        }
    }
}

pub async fn handle_request(connection: &mut ConnectionClient) -> Result<()> {
    let packets = connection.read().await?;
    let PacketKind::RequestNetworkSetting(request) = &packets[0] else {
        bail!("login_process packet missmatch,expected:RequestNetworkSetting")
    };
    match request.client_protocol {
        n if n < PROTOCOL_VERSION => {
            connection
                .write(PlayStatusPacket::FailedClient.into())
                .await?
        }
        n if n > PROTOCOL_VERSION => {
            connection
                .write(PlayStatusPacket::FailedSpawn.into())
                .await?
        }
        _ => {
            let network_setting = NetworkSettingsPacket {
                compression_threshold: 512,
                compression_algorithm: CompressionAlgorithmType::Deflate,
                client_throttle: false,
                client_throttle_threshold: 0,
                client_throttle_scalar: 0.0,
            };
            connection.write(network_setting.into()).await?;
            connection.enable_compression();
            return Ok(());
        }
    }
    bail!("protocol version missmatch");
}

pub async fn handle_login(
    connection: &mut ConnectionClient,
) -> Result<(Box<SkinData>, ExtraUserdata)> {
    let packet = connection.read().await?;
    if let Some(PacketKind::Login(login)) = packet.into_iter().next() {
        let LoginProcess {
            skin,
            secret_key,
            token,
            user_data,
        } = connection
            .runtime
            .spawn_blocking(|| LoginProcess::verify(login))
            .await??;
        connection
            .write(ServerToClientHandshakePacket { token }.into())
            .await?;
        connection.enable_encryption(&secret_key);
        return Ok((skin, user_data));
    }
    bail!("login_process packet missmatch,expected:Login")
}
struct LoginProcess {
    skin: Box<SkinData>,
    secret_key: [u8; 32],
    token: String,
    user_data: ExtraUserdata,
}
impl LoginProcess {
    pub fn verify(login: LoginPacket) -> Result<LoginProcess> {
        let (pubkey, user_data) = verify_login(&login.identity)?;
        let skin = verify_skin(&pubkey, &login.client)?;
        let (secret_key, token) = shared_secret(&pubkey)?;
        Ok(LoginProcess {
            skin: Box::new(skin),
            secret_key,
            token,
            user_data,
        })
    }
}
