use from_num::from_num;
use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct PlayerName {
    pub xuid: String,
    pub client_uuid: String,
    pub user_name: String,
}
impl Component for PlayerName {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct RunTimeID {
    pub id: u64,
}
impl Component for RunTimeID {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
#[from_num(u8)]
pub enum DeviceOS {
    Undefined,
    Android,
    IOS,
    OSX,
    FireOS,
    GearVR,
    Hololens,
    Win10,
    Win32,
    Dedicated,
    TVOS,
    Orbis,
    NintendoSwitch,
    Xbox,
    WindowsPhone,
    Linux,
}
impl Component for DeviceOS {
    type Storage = VecStorage<Self>;
}
