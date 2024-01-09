use proto_bytes::{BufMut, ConditionalWriter};

use super::Packet;

#[derive(Debug)]
pub struct ResourcePacksStackPacket {
    pub must_accept: bool,
    pub behavior_packs: Vec<StackPackIdVersion>,
    pub resource_packs: Vec<StackPackIdVersion>,
    pub game_version: String,
    pub experiments: Vec<StackExperiment>,
    pub experiments_previously_used: bool,
}

impl Packet for ResourcePacksStackPacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_bool(self.must_accept);
        bytes.put_varint(self.behavior_packs.len() as u64);
        for p in self.behavior_packs.iter() {
            bytes.put_string_varint(&p.name);
            bytes.put_string_varint(&p.version);
            bytes.put_string_varint(&p.uuid);
        }
        bytes.put_varint(self.resource_packs.len() as u64);
        for p in self.resource_packs.iter() {
            bytes.put_string_varint(&p.name);
            bytes.put_string_varint(&p.version);
            bytes.put_string_varint(&p.uuid);
        }
        bytes.put_string_varint(&self.game_version);
        bytes.put_i32_le(self.experiments.len() as i32);
        for experiment in self.experiments.iter() {
            bytes.put_string_varint(&experiment.name);
            bytes.put_bool(experiment.enabled);
        }
        bytes.put_bool(self.experiments_previously_used);
        Ok(())
    }
}

#[derive(Debug)]
pub struct StackPackIdVersion {
    pub uuid: String,
    pub version: String,
    pub name: String,
}

#[derive(Debug)]
pub struct StackExperiment {
    pub name: String,
    pub enabled: bool,
}
