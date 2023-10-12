use anyhow::Result;
use protodef::prelude::*;

use crate::packet_ids;

#[derive(Debug)]
pub struct ResourcePacksStackPacket {
    pub must_accept: bool,
    pub behavior_packs: Vec<PackIdVersion>,
    pub resource_packs: Vec<PackIdVersion>,
    pub game_version: String,
    pub experiments: Vec<Experiment>,
    pub is_experimental: bool,
}
impl ResourcePacksStackPacket {
    pub fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_bool(self.must_accept)?;
        vec.write_varint(self.behavior_packs.len() as u64)?;
        for PackIdVersion { uuid, version, name } in self.behavior_packs.iter() {
            vec.write_string(uuid)?;
            vec.write_string(version)?;
            vec.write_string(name)?;
        }
        vec.write_varint(self.resource_packs.len() as u64)?;
        for PackIdVersion { uuid, version, name } in self.resource_packs.iter() {
            vec.write_string(uuid)?;
            vec.write_string(version)?;
            vec.write_string(name)?;
        }
        vec.write_string(&self.game_version)?;
        vec.write_li32(self.experiments.len() as i32)?;
        for experiment in self.experiments.iter() {
            vec.write_string(&experiment.name)?;
            vec.write_bool(experiment.enabled)?;
        }
        vec.write_bool(self.is_experimental)?;
        Ok(())
    }
}
packet_ids!(ResourcePacksStackPacket, 7, "resource_pack_stack_packet");

#[derive(Debug)]
pub struct PackIdVersion {
    uuid: String,
    version: String,
    name: String,
}

#[derive(Debug)]
pub struct Experiment {
    name: String,
    enabled: bool,
}
