use anyhow::Context;
use proto_bytes::ConditionalBufMut;

use super::Packet;

#[derive(Debug)]
pub struct DisconnectPacket {
    pub reason: DisconnectFailReason,
    pub hide_message: bool,
    pub message: Option<String>,
}
impl DisconnectPacket {
    pub fn from_str(message: &str) -> Self {
        Self {
            reason: DisconnectFailReason::Unknown,
            hide_message: false,
            message: Some(message.to_owned()),
        }
    }
}
impl Packet for DisconnectPacket {
    fn decode(_bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    #[inline]
    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_zigzag32(self.reason.clone() as i32);
        bytes.put_bool(self.hide_message);
        if !self.hide_message {
            bytes.put_string_varint(
                self.message
                    .as_ref()
                    .context("Unknown DisconnectMessage.")?,
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum DisconnectFailReason {
    Unknown,
    CantConnectInternet,
    NoPermissions,
    UnrecoverableError,
    ThirdPartyBlocked,
    ThirdPartyNoInternet,
    ThirdPartyBadIp,
    ThirdPartyNoServerOrServerLocked,
    VersionMismatch,
    SkinIssue,
    InviteSessionNotFound,
    EduLevelSettingsMissing,
    LocalServerNotFound,
    LegacyDisconnect,
    UserLeaveGameAttempted,
    PlatformLockedSkinsError,
    RealmsWorldUnassgined,
    RealmsServerCantConnect,
    RealmsServerHidden,
    RealmsServerDisabledBeta,
    RealmsServerDisabled,
    CrossPlatformDisallowed,
    CantConnect,
    SessionNotFound,
    ClientSettingsIncompatibleWithServer,
    ServerFull,
    InvalidPlatformSkin,
    EditionVersionMismatch,
    EditionMismatch,
    LevelNewerThanExeVersion,
    NoFailOccurred,
    BannedSkin,
    Timeout,
    ServerNotFound,
    OutdatedServer,
    OutdatedClient,
    NoPremiumPlatform,
    MultiplayerDisabled,
    NoWifi,
    WorldCorruption,
    NoReason,
    Disconnected,
    InvalidPlayer,
    LoggedInOtherLocation,
    ServerIdConflict,
    NotAllowed,
    NotAuthenticated,
    InvalidTenant,
    UnknownPacket,
    UnexpectedPacket,
    InvalidCommandRequestPacket,
    HostSuspended,
    LoginPacketNoRequest,
    LoginPacketNoCert,
    MissingClient,
    Kicked,
    KickedForExploit,
    KickedForIdle,
    ResourcePackProblem,
    IncompatiblePack,
    OutOfStorage,
    InvalidLevel,
    DisconnectPacketDeprecated,
    BlockMismatch,
    InvalidHeights,
    InvalidWidths,
    ConnectionLost,
    ZombieConnection,
    Shutdown,
    ReasonNotSet,
    LoadingStateTimeout,
    ResourcePackLoadingFailed,
    SearchingForSessionLoadingScreenFailed,
    ConnProtocolVersion,
    SubsystemStatusError,
    EmptyAuthFromDiscovery,
    EmptyUrlFromDiscovery,
    ExpiredAuthFromDiscovery,
    UnknownSignalServiceSignInFailure,
    XblJoinLobbyFailure,
    UnspecifiedClientInstanceDisconnection,
    ConnSessionNotFound,
    ConnCreatePeerConnection,
    ConnIce,
    ConnConnectRequest,
    ConnConnectResponse,
    ConnNegotiationTimeout,
    ConnInactivityTimeout,
    StaleConnectionBeingReplaced,
    RealmsSessionNotFound,
    BadPacket,
}
