use proto_bytes::ConditionalWriter;

use super::Packet;

#[derive(Debug)]
pub struct DisconnectPacket {
    pub reason: DisconnectFailReason,
    pub hide_disconnect_reason: bool,
    pub message: String,
}

impl Packet for DisconnectPacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    #[inline]
    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_zigzag32(self.reason.clone() as i32);
        bytes.put_bool(self.hide_disconnect_reason);
        if !self.hide_disconnect_reason {
            bytes.put_string_varint(&self.message);
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
