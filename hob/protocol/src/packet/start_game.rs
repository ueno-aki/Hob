use std::collections::HashMap;

use anyhow::anyhow;
use hob_nbt::VarInt;
use proto_bytes::{BufMut, ConditionalBufMut};
use uuid::Uuid;

use super::Packet;

#[derive(Debug)]
pub struct StartGamePacket {
    entity_id: i64,
    runtime_id: u64,
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
    is_world_template_settings_locked: bool,
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
    world_template_id: Uuid,
    client_side_generation: bool,
    block_network_ids_are_hashes: bool,
    server_controlled_sound: bool,
}
impl StartGamePacket {
    pub fn new(runtime_id: u64, game_mode: GameMode) -> Self {
        Self {
            entity_id: runtime_id as i64,
            runtime_id,
            gamemode: game_mode,
            player_position: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0),
            seed: 0,
            biome_type: 0,
            biome_name: "".to_string(),
            dimension: Dimension::OverWorld,
            generator: 0,
            world_gamemode: game_mode,
            difficulty: 0,
            spawn_position: (0, 0, 0),
            achievements_disabled: false,
            editor_world_type: EditorWorldType::NotEditor,
            created_in_editor: false,
            exported_from_editor: false,
            day_cycle_stop_time: 0,
            education_offer: 0,
            education_features_enabled: false,
            education_product_uuid: "".to_string(),
            rain_level: 0.0,
            lightning_level: 0.0,
            has_confirmed_platform_locked_content: false,
            is_multiplayer: false,
            broadcast_to_lan: false,
            xbox_live_broadcast_mode: 0,
            platform_broadcast_mode: 0,
            enable_commands: false,
            is_texturepacks_required: false,
            gamerules: Vec::new(),
            experiments: Vec::new(),
            experiments_previously_used: false,
            bonus_chest: false,
            map_enabled: false,
            permission_level: PermissionLevel::Member,
            server_chunk_tick_range: 0,
            has_locked_behavior_pack: false,
            has_locked_resource_pack: false,
            is_from_locked_world_template: false,
            msa_gamertags_only: false,
            is_from_world_template: false,
            is_world_template_settings_locked: false,
            only_spawn_v1_villagers: false,
            persona_disabled: false,
            custom_skins_disabled: false,
            emote_chat_muted: false,
            game_version: "*".to_string(),
            limited_world_width: 0,
            limited_world_length: 0,
            is_new_nether: false,
            edu_resource_uri: EducationSharedResourceURI::default(),
            experimental_gameplay_override: false,
            chat_restriction_level: ChatRestrictionLevel::None,
            disable_player_interactions: false,
            level_id: "".to_string(),
            world_name: "".to_string(),
            premium_world_template_id: "".to_string(),
            is_trial: false,
            movement_authority: MovementAuthority::Client,
            rewind_history_size: 0,
            server_authoritative_block_breaking: false,
            current_tick: 0,
            enchantment_seed: 0,
            block_properties: Vec::new(),
            itemstates: Vec::new(),
            multiplayer_correlation_id: "".to_string(),
            server_authoritative_inventory: false,
            engine: "".to_string(),
            property_data: hob_nbt::value::Value::Compound(HashMap::new()),
            block_pallette_checksum: 0,
            world_template_id: Uuid::nil(),
            client_side_generation: false,
            block_network_ids_are_hashes: false,
            server_controlled_sound: false,
        }
    }
}

impl Packet for StartGamePacket {
    fn decode(_bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    // https://github.com/Sandertv/gophertunnel/blob/master/minecraft/protocol/packet/start_game.go

    fn encode(&self, bytes: &mut proto_bytes::BytesMut) -> anyhow::Result<()> {
        bytes.put_zigzag64(self.entity_id);
        bytes.put_varint(self.runtime_id);
        bytes.put_zigzag32(self.gamemode as i32);
        {
            let (x, y, z) = self.player_position;
            bytes.put_f32_le(x);
            bytes.put_f32_le(y);
            bytes.put_f32_le(z);
        }
        {
            let (x, z) = self.rotation;
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
            let (x, y, z) = self.spawn_position;
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
        {
            let EducationSharedResourceURI {
                button_name,
                link_uri,
            } = &self.edu_resource_uri;
            bytes.put_string_varint(button_name);
            bytes.put_string_varint(link_uri);
        }
        bytes.put_bool(self.experimental_gameplay_override);
        bytes.put_u8(self.chat_restriction_level as u8);
        bytes.put_bool(self.disable_player_interactions);
        bytes.put_string_varint(&self.level_id);
        bytes.put_string_varint(&self.world_name);
        bytes.put_string_varint(&self.premium_world_template_id);
        bytes.put_bool(self.is_trial);
        bytes.put_zigzag32(self.movement_authority as i32);
        bytes.put_zigzag32(self.rewind_history_size);
        bytes.put_bool(self.server_authoritative_block_breaking);
        bytes.put_i64_le(self.current_tick);
        bytes.put_zigzag32(self.enchantment_seed);
        bytes.put_varint(self.block_properties.len() as u64);
        for BlockProperty { name, state } in self.block_properties.iter() {
            bytes.put_string_varint(name);
            bytes.put_slice(&VarInt::to_vec(state).map_err(|e| anyhow!("{e}"))?);
        }
        bytes.put_varint(self.itemstates.len() as u64);
        for ItemState {
            name,
            runtime_id,
            component_based,
        } in self.itemstates.iter()
        {
            bytes.put_string_varint(name);
            bytes.put_i16_le(*runtime_id);
            bytes.put_bool(*component_based);
        }
        bytes.put_string_varint(&self.multiplayer_correlation_id);
        bytes.put_bool(self.server_authoritative_inventory);
        bytes.put_string_varint(&self.engine);
        bytes.put_slice(&VarInt::to_vec(&self.property_data).map_err(|e| anyhow!("{e}"))?);
        bytes.put_u64_le(self.block_pallette_checksum);
        {
            let (most_sig, least_sig) = self.world_template_id.as_u64_pair();
            bytes.put_u64_le(most_sig);
            bytes.put_u64_le(least_sig);
        }
        bytes.put_bool(self.client_side_generation);
        bytes.put_bool(self.block_network_ids_are_hashes);
        bytes.put_bool(self.server_controlled_sound);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    FallBack, // fall back to the game mode set in the WorldGameMode field.
    Spectator,
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    OverWorld,
    Nether,
    End,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum PermissionLevel {
    Visitor,
    Member,
    Operator,
    Custom,
}

#[derive(Debug, Default)]
pub struct EducationSharedResourceURI {
    button_name: String,
    link_uri: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ChatRestrictionLevel {
    None,
    Dropped,
    Disabled,
}

#[derive(Debug, Clone, Copy)]
pub enum MovementAuthority {
    Client,
    Server,
    ServerWithRewind,
}

#[derive(Debug)]
pub struct BlockProperty {
    name: String,
    state: hob_nbt::value::Value,
}

#[derive(Debug)]
pub struct ItemState {
    name: String,
    runtime_id: i16,
    component_based: bool,
}
