use crate::define::{
    IEvent,
    IEventReader,
    IEventSelector,
    IEventSink,
    IEventTransformer,
    IEventWorker,
    IEventWriter,
};

pub struct EventSink<'a> {
    pub(crate) selectors: Vec<Box<&'a dyn IEventSelector>>,
    pub(crate) transformers: Vec<Box<&'a dyn IEventTransformer>>,
    pub(crate) writer: Box<&'a dyn IEventWriter>,
}

impl IEventSink for EventSink<'_> {
    fn next(&self, event: Box<dyn IEvent>) {
        let mut event = event;
        for selector in &self.selectors {
            if !selector.select(Box::new(event.as_ref())) {
                return;
            }
        }
        // transform
        for transformer in &self.transformers {
            if let Some(new_event) = transformer.transform(Box::new(event.as_ref())) {
                event = new_event;
            }
        }
        self.writer.write(event);
    }
}

pub struct EventWorker {
    name: String,
    reader: Box<dyn IEventReader>,
    writer: Box<dyn IEventWriter>,
    selectors: Vec<Box<dyn IEventSelector>>,
    transformers: Vec<Box<dyn IEventTransformer>>,
}

impl IEventWorker for EventWorker {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) {
        let mut selectors: Vec<Box<&dyn IEventSelector>> = Vec::new();
        for selector in &self.selectors {
            selectors.push(Box::new(selector.as_ref()));
        }
        let mut transformers: Vec<Box<&dyn IEventTransformer>> = Vec::new();
        for transformer in &self.transformers {
            transformers.push(Box::new(transformer.as_ref()));
        }
        let writer = Box::new(self.writer.as_ref());
        let sink = EventSink {
            writer,
            selectors,
            transformers,
        };
        self.reader.read(Box::new(&sink));
    }
}

pub struct EventWorkerBuilder {
    name: String,
    reader: Option<Box<dyn IEventReader>>,
    writer: Option<Box<dyn IEventWriter>>,
    filters: Vec<Box<dyn IEventSelector>>,
    transformers: Vec<Box<dyn IEventTransformer>>,
}

impl EventWorkerBuilder {
    pub fn new(name: &str) -> Self {
        EventWorkerBuilder {
            name: name.to_string(),
            reader: None,
            writer: None,
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

    pub fn build(&mut self) -> Box<dyn IEventWorker> {
        Box::new(EventWorker {
            name: self.name.clone(),
            reader: self.reader.take().expect("reader is not set"),
            writer: self.writer.take().expect("writer is not set"),
            selectors: Vec::new(),
            transformers: Vec::new(),
        })
    }
}