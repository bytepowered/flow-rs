use async_trait::async_trait;

use crate::define::{IEvent, IEventBuilder, IEventInput, IEventOutput, IEventSink};
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

#[async_trait]
impl IEventInput for IterableEventReader {
    async fn read<'a>(&self, sink: Box<&'a dyn IEventSink>) -> Result<(), anyhow::Error> {
        for i in 0..self.iter {
            let mut builder = EventBuilder::new();
            let event = builder
                .id(i)
                .source(format!("evt-{}", i).as_str())
                .tag(i)
                .timestamp(1)
                .kind(0)
                .build();
            sink.next(event).await?;
        }
        Ok(())
    }
}

unsafe impl Send for IterableEventReader {}
unsafe impl Sync for IterableEventReader {}

pub struct ConsoleEventWriter {}

impl ConsoleEventWriter {
    pub fn new() -> Self {
        ConsoleEventWriter {}
    }
}

#[async_trait]
impl IEventOutput for ConsoleEventWriter {
    async fn write(&self, event: Box<dyn IEvent>)  -> Result<(), anyhow::Error> {
        println!("{} {} {} {} {}", event.id(), event.source(), event.tag(), event.timestamp(), event.kind());
        Ok(())
    }
}

unsafe impl Send for ConsoleEventWriter {}
unsafe impl Sync for ConsoleEventWriter {}
