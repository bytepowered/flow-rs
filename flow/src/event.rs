use crate::define::{
    IEventBuilder,
    IEvent,
};

pub struct Event {
    pub id: u64,
    pub source: String,
    pub tag: u64,
    pub timestamp: u64,
    pub kind: u16,
}

impl IEvent for Event {
    fn id(&self) -> u64 {
        self.id
    }

    fn source(&self) -> &str {
        &self.source
    }

    fn tag(&self) -> u64 {
        self.tag
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn kind(&self) -> u16 {
        self.kind
    }
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

pub struct EventBuilder {
    id: u64,
    source: String,
    tag: u64,
    timestamp: u64,
    kind: u16,
}

impl IEventBuilder for EventBuilder {
    fn id(&mut self, id: u64) -> &mut Self {
        self.id = id;
        self
    }

    fn source(&mut self, source: &str) -> &mut Self {
        self.source = source.to_string();
        self
    }

    fn tag(&mut self, tag: u64) -> &mut Self {
        self.tag = tag;
        self
    }

    fn timestamp(&mut self, timestamp: u64) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    fn kind(&mut self, kind: u16) -> &mut Self {
        self.kind = kind;
        self
    }

    fn build(&self) -> Box<dyn IEvent> {
        Box::new(Event {
            id: self.id,
            source: self.source.clone(),
            tag: self.tag,
            timestamp: self.timestamp,
            kind: self.kind,
        })
    }
}

impl EventBuilder {
    pub fn new() -> Self {
        EventBuilder {
            id: 0,
            source: String::new(),
            tag: 0,
            timestamp: 0,
            kind: 0,
        }
    }
}
