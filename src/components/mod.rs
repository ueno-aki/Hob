#[derive(Debug, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
pub struct PlayerName {
    pub xuid: String,
    pub client_uuid: String,
    pub user_name: String,
}

macro_rules! os {
    ($($name:tt),*) => {
        #[derive(Debug)]
        pub enum DeviceOS {
            $($name,)*
        }

        impl From<u8> for DeviceOS{
            fn from(value: u8) -> Self {
                match value {
                    $(n if n == DeviceOS::$name as u8 => DeviceOS::$name,)*
                    _ => panic!("Unspecified DevicsOS")
                }
            }
        }
    }
}
os![
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
    Linux
];
