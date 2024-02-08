use anyhow::{anyhow, Result};
use hob_protocol::packet::{
    disconnect::DisconnectPacket,
    handshake::{shared_secret, ServerToClientHandshakePacket},
    login::{verify_login, verify_skin, ExtraUserdata, SkinData},
    network_settings::{CompressionAlgorithmType, NetworkSettingsPacket},
    play_status::PlayStatusPacket,
    PacketKind,
};
use log::debug;

use crate::connection_client::ConnectionClient;

#[derive(Debug)]
pub enum LoginResult {
    Success(Box<SkinData>, ExtraUserdata),
    Failed,
}

static PROTOCOL_VERSION: i32 = 649;

pub async fn login_process(connection: &mut ConnectionClient) -> Result<LoginResult> {
    if let Err(e) = handle_request(connection).await {
        debug!("login_process failed: {:?}", e);
        return Ok(LoginResult::Failed);
    }

    match handle_login(connection).await {
        Ok((skin, userdata)) => Ok(LoginResult::Success(skin, userdata)),
        Err(e) => {
            debug!("login_process failed: {:?}", e);
            connection
                .write(DisconnectPacket::from("disconnectionScreen.notAuthenticated").into())
                .await?;
            Ok(LoginResult::Failed)
        }
    }
}

pub async fn handle_request(connection: &mut ConnectionClient) -> Result<()> {
    let packets = connection.read().await?;
    let PacketKind::RequestNetworkSetting(request) = &packets[0] else {
        return Err(anyhow!(
            "login_process packet missmatch,expected:RequestNetworkSetting"
        ));
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
            connection.writer.write(network_setting.into()).await?;
            connection.enable_compression();
            return Ok(());
        }
    }
    Err(anyhow!("login_process protocol missmatch"))
}

pub async fn handle_login(
    connection: &mut ConnectionClient,
) -> Result<(Box<SkinData>, ExtraUserdata)> {
    let PacketKind::Login(login) = &connection.read().await?[0] else {
        return Err(anyhow!("login_process packet missmatch,expected:Login"));
    };
    let (pubkey, client_data) = verify_login(&login.identity)?;
    let skin = verify_skin(&pubkey, &login.client)?;
    let (secret, token) = shared_secret(&pubkey)?;
    connection
        .write(ServerToClientHandshakePacket { token }.into())
        .await?;

    connection.enable_encryption(&secret);

    Ok((Box::new(skin), client_data))
}
