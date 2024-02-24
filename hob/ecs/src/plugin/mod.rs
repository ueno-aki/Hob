use std::sync::atomic::{AtomicBool, Ordering};

use specs::{prelude::*, rayon::iter::IntoParallelRefMutIterator};

#[derive(Default)]
pub struct Plugin<E> {
    plugins: Vec<Box<dyn for<'a> EventRunNow<'a, E> + Send + Sync>>,
}

impl<E> Plugin<E>
where
    E: Send + Sync,
{
    pub fn new() -> Self {
        Plugin {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<T>(&mut self, plugin: T)
    where
        T: for<'a> PluginSys<'a, E> + Send + Sync + 'static,
    {
        self.plugins.push(Box::new(plugin));
    }

    pub fn run<'a>(&mut self, event: &'a E, world: &'a World) -> bool {
        let cancel = AtomicBool::new(false);
        self.plugins.par_iter_mut().for_each(|plugin| {
            if plugin.run_now(event, world) {
                cancel.fetch_and(true, Ordering::Relaxed);
            }
        });
        cancel.load(Ordering::Relaxed)
    }
}

pub trait PluginSys<'a, E> {
    type SystemData: SystemData<'a>;
    fn run(&mut self, event: &'a E, data: Self::SystemData) -> bool;
}

pub trait EventRunNow<'a, E> {
    fn run_now(&mut self, event: &'a E, world: &'a World) -> bool;
}

impl<'a, E, T> EventRunNow<'a, E> for T
where
    T: PluginSys<'a, E>,
{
    fn run_now(&mut self, event: &'a E, world: &'a World) -> bool {
        let data = T::SystemData::fetch(world);
        self.run(event, data)
    }
}
