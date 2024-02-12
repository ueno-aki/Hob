use proto_bytes::{BufMut, ConditionalBufMut};

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
impl Default for ResourcePacksStackPacket {
    fn default() -> Self {
        Self {
            must_accept: false,
            behavior_packs: vec![],
            resource_packs: vec![],
            game_version: String::from("*"),
            experiments: vec![],
            experiments_previously_used: false,
        }
    }
}
impl ResourcePacksStackPacket {
    pub fn new(
        must_accept: bool,
        behavior_packs: Vec<StackPackIdVersion>,
        resource_packs: Vec<StackPackIdVersion>,
        game_version: &str,
        experiments: Vec<StackExperiment>,
        experiments_previously_used: bool,
    ) -> Self {
        Self {
            must_accept,
            behavior_packs,
            resource_packs,
            game_version: game_version.to_owned(),
            experiments,
            experiments_previously_used,
        }
    }
    pub fn add_experiment(&mut self, name: &str, enabled: bool) {
        self.experiments.push(StackExperiment::new(name, enabled));
    }
}

impl Packet for ResourcePacksStackPacket {
    fn decode(_bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
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
impl StackExperiment {
    pub fn new(name: &str, enabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            enabled,
        }
    }
}
