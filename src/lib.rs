use bevy::prelude::{App, Event};

pub mod any_event_writer;
pub mod any_res_mut;
pub mod async_event_writer;
pub mod asynchronous;
#[cfg(feature = "bevy_pkv")]
pub mod tracked_resource;
pub mod window_size;

pub mod insets;
pub mod layout;
pub mod click;

pub trait TrackableResource:
    bevy::prelude::Resource + serde::Serialize + serde::de::DeserializeOwned + Clone
{
    const KEY: &'static str;

    /// Optional function that is called when the resource is loaded
    fn on_loaded(&mut self) {}
}

pub trait CanInitTrackedResource {
    fn init_tracked_resource<R: TrackableResource + Default>(&mut self) -> &mut Self;

    fn insert_tracked_resource<R: TrackableResource>(&mut self, initial_value: R) -> &mut Self;
}

impl CanInitTrackedResource for App {
    fn init_tracked_resource<R: TrackableResource + Default>(&mut self) -> &mut Self {
        #[cfg(feature = "bevy_pkv")]
        self.add_plugins(crate::tracked_resource::TrackedResourcePlugin::<R>::default());
        #[cfg(not(feature = "bevy_pkv"))]
        self.init_resource::<R>();
        self
    }

    fn insert_tracked_resource<R: TrackableResource>(&mut self, initial_value: R) -> &mut Self {
        #[cfg(feature = "bevy_pkv")]
        self.add_plugins(crate::tracked_resource::TrackedResourcePlugin::<R>::new(
            initial_value,
        ));
        #[cfg(not(feature = "bevy_pkv"))]
        self.insert_resource::<R>(initial_value);

        self
    }
}

pub trait CanRegisterAsyncEvent {
    fn register_async_event<E: Event>(&mut self) -> &mut Self;
}

impl CanRegisterAsyncEvent for App {
    fn register_async_event<E: Event>(&mut self) -> &mut Self {
        self.add_plugins(crate::async_event_writer::AsyncEventPlugin::<E>::default());
        self
    }
}
