use crate::define::{
    IEvent,
    IEventTransformer,
};

pub struct EventTransformer {}

impl EventTransformer {
    pub fn new() -> Self {
        EventTransformer {}
    }
}

impl IEventTransformer for EventTransformer {
    fn transform(&self, event: Box<dyn IEvent>) -> Option<Box<dyn IEvent>> {
        let mut builder = BaseEventBuilder::new();
        Some(
            builder
                .id(event.id())
                .source(event.source())
                .tag(event.tag())
                .timestamp(event.timestamp())
                .kind(event.kind() + 1)
                .build()
        )
    }
}
