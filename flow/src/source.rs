use crate::define::{IEventBuilder, IEventReader, IEventSink};
use crate::event::EventBuilder;

pub struct IterableEventReader {
    name: String,
    iter: u64,
}

impl IterableEventReader {
    pub fn new(name: &str, iter: u64) -> Self {
        IterableEventReader {
            name: name.to_string(),
            iter,
        }
    }
}

impl IEventReader for IterableEventReader {
    fn read(&self, sink: Box<dyn IEventSink>) {
        for i in 0..self.iter {
            let mut builder = EventBuilder::new();
            let event = builder
                .id(i)
                .source(format!("{}-{}", self.name, i).as_str())
                .tag(i)
                .timestamp(1)
                .kind(0)
                .build();
            sink.next(event);
        }
    }
}
