pub struct StartGamePacket {
    entity_id: u64,
    runtime_entity_id: u64,
    gamemode: GameMode,
    player_position: (f32, f32, f32),
    rotation: (f32, f32),
    seed: u64,
    biome_type: i16,
    biome_name: String,
    dimension: Dimension,
    generator: u32,
    world_gamemode: GameMode,
    difficulty: u32,
    spawn_position: (u32, u64, u32),
    achievements_disabled: bool,
    editor_world_type: EditorWorldType,
    created_in_editor: bool,
    exported_from_editor: bool,
    day_cycle_stop_time: u32,
    edu_offer: u32,
    edu_features_enabled: bool,
    edu_product_uuid: String,
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
    property_data:hob_nbt::value::Value,
    block_pallette_checksum: u64,
    // world_template_id:UUID,
    client_side_generation: bool,
    block_network_ids_are_hashes: bool,
    server_controlled_sound: bool,
}

pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    FallBack,
    Spectator,
}

pub enum Dimension {
    OverWorld,
    Nether,
    End,
}

pub enum EditorWorldType {
    NotEditor,
    Project,
    TestLevel,
}

pub struct GameRule {
    name: String,
    editable: bool,
    value: GameRuleTypes,
}
pub enum GameRuleTypes {
    Int(i32),
    Bool(bool),
    Float(f32),
    Void,
}
pub struct Experiment {
    name: String,
    enable: bool,
}
pub enum PermissionLevel {
    Visitor,
    Member,
    Operator,
    Custom,
}
pub struct EducationSharedResourceURI {
    button_name: String,
    link_uri: String,
}
pub enum ChatRestrictionLevel {
    None,
    Dropped,
    Disabled,
}
pub enum MovementAuthority {
    Client,
    Server,
    ServerWithRewind,
}
pub struct BlockProperty {
    name: String,
    // state:NBT
}
pub struct ItemState {
    name: String,
    runtime_id: i16,
    component_based: bool,
}
