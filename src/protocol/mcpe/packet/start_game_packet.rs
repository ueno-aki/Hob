pub struct StartGamePacket {
    entity_id:u64,
    runtime_entity_id:u64,
    gamemode:GameMode,
    player_position:(f32,f32,f32),
    rotation:(f32,f32),
    seed:u64,
    biome_type:i16,
    biome_name:String,
    dimension:Dimension,
    generator:u32,
    world_gamemode:GameMode,
    difficulty:u32,
    spawn_position:(u32,u64,u32),
    achievements_disabled:bool,
    editor_world_type:EditorWorldType,
    created_in_editor:bool,
    exported_from_editor:bool,
    day_cycle_stop_time:u32,
    edu_offer:u32,
    edu_features_enabled:bool,
    edu_product_uuid:String,
    rain_level:f32,
    lightning_level:f32,
    has_confirmed_platform_locked_content:bool,
    is_multiplayer:bool,
    broadcast_to_lan:bool,
    xbox_live_broadcast_mode:u64,
    platform_broadcast_mode:u64,
    enable_commands:bool,
    is_texturepacks_required:bool,
}

pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    FallBack,
    Spectator
}

pub enum Dimension {
    OverWorld,
    Nether,
    End
}

pub enum EditorWorldType {
    NotEditor,
    Project,
    TestLevel
}