use crate::define::{
    IEvent,
    IEventSelector,
};

pub struct EventSelector {
    kind: u16,
}

impl EventSelector {
    pub fn new(kind: u16) -> Self {
        EventSelector {
            kind,
        }
    }
}

impl IEventSelector for EventSelector {
    fn select(&self, event: Box<&dyn IEvent>) -> bool {
        event.kind() == self.kind
    }
}
