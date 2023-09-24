use anyhow::Result;
use protodef::prelude::*;

use crate::packet_feature;

#[derive(Debug)]
pub struct ResourcePacksInfoPacket {
    pub must_accept:bool,
    pub scripting:bool,
    pub force_server_packs:bool,
    pub behaviour_pack_infos:Vec<BehaviourPackInfo>,
    pub resource_pack_infos:Vec<ResourcePackInfo>
}
impl ResourcePacksInfoPacket {
    pub fn read_to_buffer(&self, vec: &mut Vec<u8>) -> Result<()> {
        vec.write_bool(self.must_accept)?;
        vec.write_bool(self.scripting)?;
        vec.write_bool(self.force_server_packs)?;
        self.encode_behavior(vec)?;
        self.encode_resouce(vec)?;
        Ok(())
    }
    fn encode_behavior(&self, vec: &mut Vec<u8>) -> Result<()>{
        for behavior in self.behaviour_pack_infos.iter() {
            vec.write_string(&behavior.uuid)?;
            vec.write_string(&behavior.version)?;
            vec.write_lu64(behavior.size)?;
            vec.write_string(&behavior.encryption_key)?;
            vec.write_string(&behavior.sub_pack_name)?;
            vec.write_string(&behavior.content_identity)?;
            vec.write_bool(behavior.scripting)?;
        }
        Ok(())
    }
    fn encode_resouce(&self, vec: &mut Vec<u8>) -> Result<()>{
        for resource in self.resource_pack_infos.iter() {
            vec.write_string(&resource.uuid)?;
            vec.write_string(&resource.version)?;
            vec.write_lu64(resource.size)?;
            vec.write_string(&resource.encryption_key)?;
            vec.write_string(&resource.sub_pack_name)?;
            vec.write_string(&resource.content_identity)?;
            vec.write_bool(resource.scripting)?;
            vec.write_bool(resource.rtx_enabled)?;
        }
        Ok(())
    }
}

packet_feature!(ResourcePacksInfoPacket,6,"resource_pack_info_packet");

#[derive(Debug)]
pub struct BehaviourPackInfo {
    pub uuid:String,
    pub version:String,
    pub size:u64,
    pub encryption_key:String,
    pub sub_pack_name:String,
    pub content_identity:String,
    pub scripting:bool
}
#[derive(Debug)]
pub struct ResourcePackInfo {
    pub uuid:String,
    pub version:String,
    pub size:u64,
    pub encryption_key:String,
    pub sub_pack_name:String,
    pub content_identity:String,
    pub scripting:bool,
    pub rtx_enabled:bool,
}