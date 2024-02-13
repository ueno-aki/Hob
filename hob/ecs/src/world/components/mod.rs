use specs::Component;

pub struct EntityRuntimeIdComponent(pub u64);
impl Component for EntityRuntimeIdComponent {
    type Storage = specs::VecStorage<Self>;
}
