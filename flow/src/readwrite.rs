use crate::define::{IEvent, IEventBuilder, IEventReader, IEventSink, IEventWriter};
use crate::event::EventBuilder;

pub struct IterableEventReader {
    iter: u64,
}

impl IterableEventReader {
    pub fn new(iter: u64) -> Self {
        IterableEventReader {
            iter,
        }
    }
}

impl IEventReader for IterableEventReader {
    fn read<'a>(&self, sink: Box<&'a dyn IEventSink>) {
        for i in 0..self.iter {
            let mut builder = EventBuilder::new();
            let event = builder
                .id(i)
                .source(format!("evt-{}", i).as_str())
                .tag(i)
                .timestamp(1)
                .kind(0)
                .build();
            sink.next(event);
        }
    }
}

pub struct ConsoleEventWriter {}

impl ConsoleEventWriter {
    pub fn new() -> Self {
        ConsoleEventWriter {}
    }
}

impl IEventWriter for ConsoleEventWriter {
    fn write(&self, event: Box<dyn IEvent>) {
        println!("{} {} {} {} {}", event.id(), event.source(), event.tag(), event.timestamp(), event.kind());
    }
}
