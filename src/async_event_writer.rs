use bevy::{ecs::system::SystemParam, prelude::*};
use std::marker::PhantomData;
use std::sync::mpsc::*;

pub(crate) struct AsyncEventPlugin<T: Event>(PhantomData<T>);

impl<T: Event> Default for AsyncEventPlugin<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Event> Plugin for AsyncEventPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<T>()
            .add_systems(Update, poll_events::<T>)
            .init_non_send_resource::<AsyncEventResource<T>>();
    }
}

fn poll_events<T: Event>(channels: NonSend<AsyncEventResource<T>>, mut writer: EventWriter<T>) {
    while let Ok(ev) = channels.receiver.try_recv() {
        writer.send(ev);
    }
}

#[derive(Debug, Clone)]
pub struct AsyncEventWriter<T: Event>(Sender<T>);

impl<T: Event> AsyncEventWriter<T> {
    pub fn send(&self, event: T) -> Result<(), SendError<T>> {
        self.0.send(event)
    }

    pub fn send_or_panic(&self, event: T) -> () {
        match self.0.send(event) {
            Ok(()) => {}
            Err(err) => {
                panic!("{err}")
            }
        }
    }
}

unsafe impl<T: Event> SystemParam for AsyncEventWriter<T> {
    type State = ();

    type Item<'world, 'state> = Self;

    fn init_state(
        _world: &mut World,
        _system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
    }

    unsafe fn get_param<'world, 'state>(
        _: &'state mut Self::State,
        _: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        _: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        match world.get_non_send_resource::<AsyncEventResource<T>>() {
            Some(resource) => Self(resource.sender.clone()),
            None => panic!(
                "Event {} is not registered as an async event",
                std::any::type_name::<T>()
            ),
        }
    }
}

#[derive(Resource)]
struct AsyncEventResource<T: Event> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T: Event> Default for AsyncEventResource<T> {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self { sender, receiver }
    }
}
