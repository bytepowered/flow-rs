use crate::define::{
    IEventSink,
    IEventReader,
};

pub struct IterableEventSource {
    name: String,
    iter: u64,
}

impl IterableEventSource {
    pub fn new(name: &str, iter: u64) -> Self {
        IterableEventSource {
            name: name.to_string(),
            iter,
        }
    }
}

impl IEventReader for IterableEventSource {

    fn read(&self, mut sink: Box<dyn IEventSink>) {
        for i in 0..self.iter {
            let mut builder = BaseEventBuilder::new();
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
