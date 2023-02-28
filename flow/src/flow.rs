use crate::define::{IEvent, IEventBuilder, IEventReader, IEventSelector, IEventSink, IEventTransformer, IEventWriter, IFlow};

pub struct EventSink<'a> {
    pub(crate) filters: &<'a> Vec<Box<dyn IEventSelector>>,
    pub(crate) transformers: Vec<Box<dyn IEventTransformer>>,
    pub(crate) writer: Box<dyn IEventWriter>,
}

impl IEventSink for EventSink {
    fn next(&mut self, event: Box<dyn IEvent>) {
        let mut event = event;
        for filter in &self.filters {
            if !filter.select(Box::new(event.as_ref())) {
                return;
            }
        }
        self.writer.write(event);
    }
}

pub struct FlowEngine {
    name: String,
    reader: Box<dyn IEventReader>,
    writer: Box<dyn IEventWriter>,
    filters: Vec<Box<dyn IEventSelector>>,
    transformers: Vec<Box<dyn IEventTransformer>>,
}

impl IFlow for FlowEngine {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&mut self) {
        self.reader.read(Box::new(EventSink {
            filters: self.filters.clone(),
            transformers: self.transformers.clone(),
            writer: self.writer.clone(),
        }));
    }
}

pub struct FlowBuilder {
    name: String,
    reader: Option<Box<dyn IEventReader>>,
    writer: Option<Box<dyn IEventWriter>>,
    sink: Option<Box<dyn IEventSink>>,
    filters: Vec<Box<dyn IEventSelector>>,
    transformers: Vec<Box<dyn IEventTransformer>>,
}

impl FlowBuilder {
    pub fn new(name: &str) -> Self {
        FlowBuilder {
            name: name.to_string(),
            reader: None,
            writer: None,
            sink: None,
            filters: Vec::new(),
            transformers: Vec::new(),
        }
    }

    pub fn reader(&mut self, reader: Box<dyn IEventReader>) -> &mut Self {
        self.reader = Some(reader);
        self
    }

    pub fn writer(&mut self, writer: Box<dyn IEventWriter>) -> &mut Self {
        self.writer = Some(writer);
        self
    }

    pub fn add_selector(&mut self, selector: Box<dyn IEventSelector>) -> &mut Self {
        self.filters.push(selector);
        self
    }

    pub fn add_transformer(&mut self, transformer: Box<dyn IEventTransformer>) -> &mut Self {
        self.transformers.push(transformer);
        self
    }

    pub fn build(&mut self) -> Box<dyn IFlow> {
        Box::new(FlowEngine {
            name: self.name.clone(),
            reader: self.reader.take().expect("reader is not set"),
            writer: self.writer.take().expect("writer is not set"),
            filters: Vec::new(),
            transformers: Vec::new(),
        })
    }
}