use bytes::BufMut;

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
    fn from_bytes(bytes: &mut bytes::BytesMut) -> anyhow::Result<Self> {
        todo!()
    }

    #[inline]
    fn read_to_bytes(&self,bytes: &mut bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_i32(self.clone() as i32);
        Ok(())
    }
}