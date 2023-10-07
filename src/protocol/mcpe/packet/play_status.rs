use anyhow::Result;
use protodef::prelude::*;

use crate::packet_ids;

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

impl PlayStatusPacket {
    pub fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_i32(self.clone() as i32)?;
        Ok(())
    }
}
packet_ids!(PlayStatusPacket, 2, "play_status_packet");
