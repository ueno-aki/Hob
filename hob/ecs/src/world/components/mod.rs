use specs::Component;

pub struct RuntimeIdComponent(pub u64);
impl Component for RuntimeIdComponent {
    type Storage = specs::VecStorage<Self>;
}
