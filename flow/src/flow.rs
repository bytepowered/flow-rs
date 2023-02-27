use crate::define::{Event, EventBuilder, EventSelector, EventSink, EventSource, EventTransformer, Flow};

pub struct BaseEvent {
    pub id: u64,
    pub source: String,
    pub tag: u64,
    pub timestamp: u64,
    pub kind: u16,
}

impl Event for BaseEvent {
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

pub struct BaseEventBuilder {
    id: u64,
    source: String,
    tag: u64,
    timestamp: u64,
    kind: u16,
}

impl EventBuilder for BaseEventBuilder {
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

    fn build(&self) -> Box<dyn Event> {
        Box::new(BaseEvent {
            id: self.id,
            source: self.source.clone(),
            tag: self.tag,
            timestamp: self.timestamp,
            kind: self.kind,
        })
    }
}

impl BaseEventBuilder {
    pub fn new() -> Self {
        BaseEventBuilder {
            id: 0,
            source: String::new(),
            tag: 0,
            timestamp: 0,
            kind: 0,
        }
    }
}

pub struct IterEventSource {
    name: String,
    count: u64,
}

impl IterEventSource {
    pub fn new(name: &str, count: u64) -> Self {
        IterEventSource {
            name: name.to_string(),
            count,
        }
    }
}

impl EventSource for IterEventSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn read(&self, mut sink: Box<dyn EventSink>) {
        for i in 0..self.count {
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

pub struct BaseEventSink {
    name: String,
}

impl EventSink for BaseEventSink {
    fn next(&mut self, event: Box<dyn Event>) {
        //println!("{}: {:?}", self.name, event);
    }
}

impl BaseEventSink {
    pub fn new(name: &str) -> Self {
        BaseEventSink {
            name: name.to_string(),
        }
    }
}

pub struct BaseEventSelector {
    name: String,
    kind: u16,
}

impl BaseEventSelector {
    pub fn new(name: &str, kind: u16) -> Self {
        BaseEventSelector {
            name: name.to_string(),
            kind,
        }
    }
}

impl EventSelector for BaseEventSelector {
    fn name(&self) -> &str {
        &self.name
    }

    fn select(&self, event: Box<dyn Event>) -> bool {
        event.kind() == self.kind
    }
}

pub struct BaseEventTransformer {
    name: String,
}

impl BaseEventTransformer {
    pub fn new(name: &str) -> Self {
        BaseEventTransformer {
            name: name.to_string(),
        }
    }
}

impl EventTransformer for BaseEventTransformer {
    fn name(&self) -> &str {
        &self.name
    }

    fn transform(&self, event: Box<dyn Event>) -> Option<Box<dyn Event>> {
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

pub struct FlowEngine {
    name: String,
    source: Box<dyn EventSource>,
    filters: Vec<Box<dyn EventSelector>>,
    transformers: Vec<Box<dyn EventTransformer>>,
}

impl Flow for FlowEngine {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&mut self) {
        // let sink =
        // let source = BaseEventSource::new("source", 100);
        // let sink = BaseEventSink::new("sink");
        // let selector = BaseEventSelector::new("selector", 0);
        // let transformer = BaseEventTransformer::new("transformer");
        //let sink: Box<EventSink> = self;
        //self.source.read(sink);
    }
}

pub struct BaseFlowBuilder {
    name: String,
    source: Option<Box<dyn EventSource>>,
    sink: Option<Box<dyn EventSink>>,
    filters: Vec<Box<dyn EventSelector>>,
    transformers: Vec<Box<dyn EventTransformer>>,
}

impl BaseFlowBuilder {
    pub fn new(name: &str) -> Self {
        BaseFlowBuilder {
            name: name.to_string(),
            source: None,
            sink: None,
            filters: Vec::new(),
            transformers: Vec::new(),
        }
    }

    pub fn source(&mut self, source: Box<dyn EventSource>) -> &mut Self {
        self.source = Some(source);
        self
    }

    pub fn sink(&mut self, sink: Box<dyn EventSink>) -> &mut Self {
        self.sink = Some(sink);
        self
    }

    pub fn add_selector(&mut self, selector: Box<dyn EventSelector>) -> &mut Self {
        self.filters.push(selector);
        self
    }

    pub fn add_transformer(&mut self, transformer: Box<dyn EventTransformer>) -> &mut Self {
        self.transformers.push(transformer);
        self
    }

    pub fn build(&mut self) -> Box<dyn Flow> {
        Box::new(FlowEngine {
            name: self.name.clone(),
            source: self.source.take().expect("source is not set"),
            filters: Vec::new(),
            transformers: Vec::new(),
        })
    }
}