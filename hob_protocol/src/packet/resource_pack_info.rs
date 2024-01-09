use anyhow::Result;
use proto_bytes::{BufMut, ConditionalWriter};

use super::Packet;

#[derive(Debug)]
pub struct ResourcePacksInfoPacket {
    pub must_accept: bool,
    pub has_scripts: bool,
    pub force_server_packs: bool,
    pub behaviour_packs: Vec<BehaviourPackInfo>,
    pub texture_packs: Vec<TexturePackInfo>,
    pub resource_pack_links: Vec<ResourcePackLink>,
}

impl Packet for ResourcePacksInfoPacket {
    fn decode(bytes: &mut proto_bytes::BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    #[inline]
    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> Result<()> {
        bytes.put_bool(self.must_accept);
        bytes.put_bool(self.has_scripts);
        bytes.put_bool(self.force_server_packs);
        self.encode_behavior(bytes)?;
        self.encode_texture(bytes)?;
        self.encode_resouce_links(bytes)?;
        Ok(())
    }
}

impl ResourcePacksInfoPacket {
    #[inline]
    fn encode_behavior(&self, bytes: &mut proto_bytes::BytesMut) -> Result<()> {
        bytes.put_i16_le(self.behaviour_packs.len() as i16);
        for behavior in self.behaviour_packs.iter() {
            bytes.put_string_varint(&behavior.uuid);
            bytes.put_string_varint(&behavior.version);
            bytes.put_u64_le(behavior.size);
            bytes.put_string_varint(&behavior.encryption_key);
            bytes.put_string_varint(&behavior.sub_pack_name);
            bytes.put_string_varint(&behavior.content_identity);
            bytes.put_bool(behavior.has_scripts);
        }
        Ok(())
    }
    #[inline]
    fn encode_texture(&self, bytes: &mut proto_bytes::BytesMut) -> Result<()> {
        bytes.put_i16_le(self.texture_packs.len() as i16);
        for resource in self.texture_packs.iter() {
            bytes.put_string_varint(&resource.uuid);
            bytes.put_string_varint(&resource.version);
            bytes.put_u64_le(resource.size);
            bytes.put_string_varint(&resource.encryption_key);
            bytes.put_string_varint(&resource.sub_pack_name);
            bytes.put_string_varint(&resource.content_identity);
            bytes.put_bool(resource.has_scripts);
            bytes.put_bool(resource.rtx_enabled);
        }
        Ok(())
    }
    #[inline]
    fn encode_resouce_links(&self, bytes: &mut proto_bytes::BytesMut) -> Result<()> {
        bytes.put_varint(self.resource_pack_links.len() as u64);
        for link in self.resource_pack_links.iter() {
            bytes.put_string_varint(&link.id);
            bytes.put_string_varint(&link.url);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct BehaviourPackInfo {
    pub uuid: String,
    pub version: String,
    pub size: u64,
    pub encryption_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
}
#[derive(Debug)]
pub struct TexturePackInfo {
    pub uuid: String,
    pub version: String,
    pub size: u64,
    pub encryption_key: String,
    pub sub_pack_name: String,
    pub content_identity: String,
    pub has_scripts: bool,
    pub rtx_enabled: bool,
}
#[derive(Debug)]
pub struct ResourcePackLink {
    pub id: String,
    pub url: String,
}
