use proto_bytes::BufMut;

use super::Packet;

#[derive(Debug, Clone)]
pub enum PlayStatusPacket {
    LoginSuccess,
    FailedClient,
    FailedSpawn,
    PlayerSpawn,
    FailedInvalidTenant,
    FailedVanillaEdu,
    FailedEduVanilla,
    FailedServerFull,
    FailedEditorVanillaMismatch,
    FailedVanillaEditorMismatch,
}

impl Packet for PlayStatusPacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self> {
        todo!()
    }

    #[inline]
    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_i32(self.clone() as i32);
        Ok(())
    }
}
