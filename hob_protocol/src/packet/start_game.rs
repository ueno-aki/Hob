use proto_bytes::{BufMut, ConditionalWriter};
use uuid::Uuid;

use super::Packet;

#[derive(Debug)]
pub struct StartGamePacket {
    entity_id: i64,
    runtime_entity_id: u64,
    gamemode: GameMode,
    player_position: (f32, f32, f32),
    rotation: (f32, f32),
    seed: u64,
    biome_type: i16,
    biome_name: String,
    dimension: Dimension,
    generator: i32,
    world_gamemode: GameMode,
    difficulty: i32,
    spawn_position: (i32, i32, i32),
    achievements_disabled: bool,
    editor_world_type: EditorWorldType,
    created_in_editor: bool,
    exported_from_editor: bool,
    day_cycle_stop_time: i32,
    education_offer: i32,
    education_features_enabled: bool,
    education_product_uuid: String,
    rain_level: f32,
    lightning_level: f32,
    has_confirmed_platform_locked_content: bool,
    is_multiplayer: bool,
    broadcast_to_lan: bool,
    xbox_live_broadcast_mode: u64,
    platform_broadcast_mode: u64,
    enable_commands: bool,
    is_texturepacks_required: bool,
    gamerules: Vec<GameRule>,
    experiments: Vec<Experiment>,
    experiments_previously_used: bool,
    bonus_chest: bool,
    map_enabled: bool,
    permission_level: PermissionLevel,
    server_chunk_tick_range: i32,
    has_locked_behavior_pack: bool,
    has_locked_resource_pack: bool,
    is_from_locked_world_template: bool,
    msa_gamertags_only: bool,
    is_from_world_template: bool,
    is_world_template_settings_locked:bool,
    only_spawn_v1_villagers: bool,
    persona_disabled: bool,
    custom_skins_disabled: bool,
    emote_chat_muted: bool,
    game_version: String,
    limited_world_width: i32,
    limited_world_length: i32,
    is_new_nether: bool,
    edu_resource_uri: EducationSharedResourceURI,
    experimental_gameplay_override: bool,
    chat_restriction_level: ChatRestrictionLevel,
    disable_player_interactions: bool,
    level_id: String,
    world_name: String,
    premium_world_template_id: String,
    is_trial: bool,
    movement_authority: MovementAuthority,
    rewind_history_size: i32,
    server_authoritative_block_breaking: bool,
    current_tick: i64,
    enchantment_seed: i32,
    block_properties: Vec<BlockProperty>,
    itemstates: Vec<ItemState>,
    multiplayer_correlation_id: String,
    server_authoritative_inventory: bool,
    engine: String,
    property_data: hob_nbt::value::Value,
    block_pallette_checksum: u64,
    world_template_id:Uuid,
    client_side_generation: bool,
    block_network_ids_are_hashes: bool,
    server_controlled_sound: bool,
}

impl Packet for StartGamePacket {
    fn decode(_bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized {
        todo!()
    }

    // https://github.com/Sandertv/gophertunnel/blob/master/minecraft/protocol/packet/start_game.go

    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_zigzag64(self.entity_id);
        bytes.put_varint(self.runtime_entity_id);
        bytes.put_zigzag32(self.gamemode as i32);
        {
            let (x,y,z) = self.player_position;
            bytes.put_f32_le(x);
            bytes.put_f32_le(y);
            bytes.put_f32_le(z);
        }
        {
            let (x,z) = self.rotation;
            bytes.put_f32_le(x);
            bytes.put_f32_le(z);
        }

        // Level Settings
        bytes.put_u64_le(self.seed);
        bytes.put_i16_le(self.biome_type);
        bytes.put_string_varint(&self.biome_name);
        bytes.put_zigzag32(self.dimension as i32);
        bytes.put_zigzag32(self.generator);
        bytes.put_zigzag32(self.world_gamemode as i32);
        bytes.put_zigzag32(self.difficulty);
        {
            let (x,y,z) = self.spawn_position;
            bytes.put_zigzag32(x);
            bytes.put_varint(y as u32 as u64); // 32 most significant bits are set to 0;
            bytes.put_zigzag32(z);
        }
        bytes.put_bool(self.achievements_disabled);
        bytes.put_zigzag32(self.editor_world_type as i32);
        bytes.put_bool(self.created_in_editor);
        bytes.put_bool(self.exported_from_editor);
        bytes.put_zigzag32(self.day_cycle_stop_time);
        bytes.put_zigzag32(self.education_offer);
        bytes.put_bool(self.education_features_enabled);
        bytes.put_string_varint(&self.education_product_uuid);
        bytes.put_f32_le(self.rain_level);
        bytes.put_f32_le(self.lightning_level);
        bytes.put_bool(self.has_confirmed_platform_locked_content);
        bytes.put_bool(self.is_multiplayer);
        bytes.put_bool(self.broadcast_to_lan);
        bytes.put_varint(self.xbox_live_broadcast_mode);
        bytes.put_varint(self.platform_broadcast_mode);
        bytes.put_bool(self.enable_commands);
        bytes.put_bool(self.is_texturepacks_required);
        bytes.put_varint(self.gamerules.len() as u64);
        for g in self.gamerules.iter() {
            g.encode_gamerule(bytes);
        }
        bytes.put_i32_le(self.experiments.len() as i32);
        for Experiment { name, enable } in self.experiments.iter() {
            bytes.put_string_varint(name);
            bytes.put_bool(*enable);
        }
        bytes.put_bool(self.experiments_previously_used);
        bytes.put_bool(self.bonus_chest);
        bytes.put_bool(self.map_enabled);
        bytes.put_u8(self.permission_level as u8);
        bytes.put_i32_le(self.server_chunk_tick_range);
        bytes.put_bool(self.has_locked_behavior_pack);
        bytes.put_bool(self.has_locked_resource_pack);
        bytes.put_bool(self.is_from_locked_world_template);
        bytes.put_bool(self.msa_gamertags_only);
        bytes.put_bool(self.is_from_world_template);
        bytes.put_bool(self.is_world_template_settings_locked);
        bytes.put_bool(self.only_spawn_v1_villagers);
        bytes.put_bool(self.persona_disabled);
        bytes.put_bool(self.custom_skins_disabled);
        bytes.put_bool(self.emote_chat_muted);
        bytes.put_string_varint(&self.game_version);
        bytes.put_i32_le(self.limited_world_width);
        bytes.put_i32_le(self.limited_world_length);
        bytes.put_bool(self.is_new_nether);
        
        todo!()
    }
}

#[derive(Debug,Clone,Copy)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    FallBack, // fall back to the game mode set in the WorldGameMode field.
    Spectator,
}

#[derive(Debug,Clone,Copy)]
pub enum Dimension {
    OverWorld,
    Nether,
    End,
}

#[derive(Debug,Clone,Copy)]
pub enum EditorWorldType {
    NotEditor,
    Project,
    TestLevel,
}

#[derive(Debug)]
pub struct GameRule {
    name: String,
    editable: bool,
    value: GameRuleTypes,
}

#[derive(Debug)]
pub enum GameRuleTypes {
    Bool(bool),
    Int(i32),
    Float(f32),
    Void,
}

impl GameRule {
    fn encode_gamerule(&self, bytes: &mut proto_bytes::BytesMut) {
        bytes.put_string_varint(&self.name);
        bytes.put_bool(self.editable);
        match self.value {
            GameRuleTypes::Bool(v) => {
                bytes.put_varint(1);
                bytes.put_bool(v);
            }
            GameRuleTypes::Int(v) => {
                bytes.put_varint(2);
                bytes.put_zigzag32(v);
            }
            GameRuleTypes::Float(v) => {
                bytes.put_varint(3);
                bytes.put_f32_le(v);
            }
            GameRuleTypes::Void => {}
        }
    }
}

#[derive(Debug)]
pub struct Experiment {
    name: String,
    enable: bool,
}

#[derive(Debug,Clone,Copy)]
pub enum PermissionLevel {
    Visitor,
    Member,
    Operator,
    Custom,
}

#[derive(Debug)]
pub struct EducationSharedResourceURI {
    button_name: String,
    link_uri: String,
}

#[derive(Debug)]
pub enum ChatRestrictionLevel {
    None,
    Dropped,
    Disabled,
}

#[derive(Debug)]
pub enum MovementAuthority {
    Client,
    Server,
    ServerWithRewind,
}

#[derive(Debug)]
pub struct BlockProperty {
    name: String,
    // state:NBT
}

#[derive(Debug)]
pub struct ItemState {
    name: String,
    runtime_id: i16,
    component_based: bool,
}
