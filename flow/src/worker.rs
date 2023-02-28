use crate::define::{
    IEvent,
    IEventInput,
    IEventSelector,
    IEventSink,
    IEventTransformer,
    IEventWorker,
    IEventOutput,
};

pub struct SynchronizedEventSink<'a> {
    pub(crate) selectors: Vec<Box<&'a dyn IEventSelector>>,
    pub(crate) transformers: Vec<Box<&'a dyn IEventTransformer>>,
    pub(crate) writer: Box<&'a dyn IEventOutput>,
}

impl IEventSink for SynchronizedEventSink<'_> {
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
    output: Box<dyn IEventInput>,
    output: Box<dyn IEventOutput>,
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
        let output = Box::new(self.output.as_ref());
        let sink = SynchronizedEventSink {
            writer: output,
            selectors,
            transformers,
        };
        self.output.read(Box::new(&sink));
    }
}

pub struct EventWorkerBuilder {
    name: String,
    input: Option<Box<dyn IEventInput>>,
    output: Option<Box<dyn IEventOutput>>,
    filters: Vec<Box<dyn IEventSelector>>,
    transformers: Vec<Box<dyn IEventTransformer>>,
}

impl EventWorkerBuilder {
    pub fn new(name: &str) -> Self {
        EventWorkerBuilder {
            name: name.to_string(),
            input: None,
            output: None,
            filters: Vec::new(),
            transformers: Vec::new(),
        }
    }

    pub fn input(&mut self, input: Box<dyn IEventInput>) -> &mut Self {
        self.input = Some(input);
        self
    }

    pub fn output(&mut self, output: Box<dyn IEventOutput>) -> &mut Self {
        self.output = Some(output);
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
            output: self.input.take().expect("reader is not set"),
            output: self.output.take().expect("writer is not set"),
            selectors: Vec::new(),
            transformers: Vec::new(),
        })
    }
}