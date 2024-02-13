pub mod connection;

use specs::Component;

pub struct DisplayNameComponent(pub String);
impl Component for DisplayNameComponent {
    type Storage = specs::VecStorage<Self>;
}

pub struct XUIDComponent(pub String);
impl Component for XUIDComponent {
    type Storage = specs::VecStorage<Self>;
}
