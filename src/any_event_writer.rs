use bevy::prelude::*;

pub trait AnyEventWriter<E: Event> {
    /// Sends an `event`, which can later be read by [`EventReader`]s.
    ///
    /// See [`Events`] for details.
    fn send(&mut self, event: E);

    /// Sends a list of `events` all at once, which can later be read by [`EventReader`]s.
    /// This is more efficient than sending each event individually.
    ///
    /// See [`Events`] for details.
    fn send_batch(&mut self, events: impl IntoIterator<Item = E>);

    /// Sends the default value of the event. Useful when the event is an empty struct.
    fn send_default(&mut self)
    where
        E: Default;
}

impl<'w, E: Event> AnyEventWriter<E> for EventWriter<'w, E> {
    fn send(&mut self, event: E) {
        self.send(event);
    }

    fn send_batch(&mut self, events: impl IntoIterator<Item = E>) {
        self.send_batch(events);
    }

    fn send_default(&mut self)
    where
        E: Default,
    {
        self.send_default();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestEventWriter<E: Event> {
    pub events: Vec<E>,
}

impl<E: Event> Default for TestEventWriter<E> {
    fn default() -> Self {
        Self {
            events: Default::default(),
        }
    }
}

impl<E: Event> AnyEventWriter<E> for TestEventWriter<E> {
    fn send(&mut self, event: E) {
        self.events.push(event);
    }

    fn send_batch(&mut self, events: impl IntoIterator<Item = E>) {
        self.events.extend(events);
    }

    fn send_default(&mut self)
    where
        E: Default,
    {
        self.events.push(E::default())
    }
}
