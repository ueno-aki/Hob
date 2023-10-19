use anyhow::Result;
use protodef::prelude::*;

use super::{Packet, PacketKind};

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
    fn from_buf(_buffer: &[u8], _offset: usize) -> Result<PacketKind> {
        unimplemented!()
    }
    fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_i32(self.clone() as i32)?;
        Ok(())
    }
}
