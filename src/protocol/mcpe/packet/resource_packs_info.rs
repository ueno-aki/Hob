pub struct ResourcePacksInfo {
    pub must_accept:bool,
    pub scripting:bool,
    pub force_server_packs:bool,
    pub behaviour_pack_infos:Vec<BehaviourPackInfo>,
    pub resource_pack_infos:Vec<ResourcePackInfo>
}
pub struct BehaviourPackInfo {
    pub uuid:String,
    pub version:String,
    pub size:u64,
    pub encryption_key:String,
    pub sub_pack_name:String,
    pub content_identity:String,
    pub scripting:bool
}
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