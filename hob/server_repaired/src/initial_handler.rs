use anyhow::{anyhow, bail, Result};
use hob_protocol::packet::{
    disconnect::DisconnectPacket,
    handshake::{shared_secret, ServerToClientHandshakePacket},
    login::{verify_login, verify_skin},
    network_settings::{CompressionAlgorithmType, NetworkSettingsPacket},
    PacketKind,
};

use crate::connection_client::ConnectionClient;

pub async fn login_process(connection: &mut ConnectionClient) -> Result<()> {
    let PacketKind::RequestNetworkSetting(_request) = &connection.read().await?[0] else {
        return Err(anyhow!(
            "login_process packet missmatch,expected:RequestNetworkSetting"
        ));
    };
    let network_setting = NetworkSettingsPacket {
        compression_threshold: 512,
        compression_algorithm: CompressionAlgorithmType::Deflate,
        client_throttle: false,
        client_throttle_threshold: 0,
        client_throttle_scalar: 0.0,
    };
    connection.writer.write(network_setting.into()).await?;
    connection.enable_compression();

    let PacketKind::Login(login) = &connection.read().await?[0] else {
        return Err(anyhow!("login_process packet missmatch,expected:Login"));
    };
    let Ok((pubkey, client_data)) = verify_login(&login.identity) else {
        connection
            .write(DisconnectPacket::from_str("disconnectionScreen.notAuthenticated").into())
            .await?;
        return Err(anyhow!("notAuthenticated"));
    };
    let skin = verify_skin(&pubkey, &login.client)?;
    let (secret, token) = shared_secret(&pubkey)?;
    connection
        .write(ServerToClientHandshakePacket { token }.into())
        .await?;
    connection.enable_encryption(&secret);

    Ok(())
}
