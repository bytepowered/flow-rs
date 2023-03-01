use crate::define::{
    IEvent,
    IEventInput,
    IEventOutput,
    IEventSelector,
    IEventSink,
    IEventTransformer,
    IEventWorker,
};

pub struct SynchronizedEventSink<'a> {
    pub(crate) selectors: Vec<Box<&'a dyn IEventSelector>>,
    pub(crate) transformers: Vec<Box<&'a dyn IEventTransformer>>,
    pub(crate) output: Box<&'a dyn IEventOutput>,
}

impl IEventSink for SynchronizedEventSink<'_> {
    fn next(&self, event: Box<dyn IEvent>) {
        let mut working_event = event;
        // Select
        for selector in &self.selectors {
            if !selector.select(Box::new(working_event.as_ref())) {
                return;
            }
        }
        // Transform
        for transformer in &self.transformers {
            if let Some(new_event) = transformer.transform(Box::new(working_event.as_ref())) {
                working_event = new_event;
            }
        }
        self.output.write(working_event);
    }
}

pub struct EventWorker {
    name: String,
    input: Box<dyn IEventInput>,
    output: Box<dyn IEventOutput>,
    selectors: Vec<Box<dyn IEventSelector>>,
    transformers: Vec<Box<dyn IEventTransformer>>,
}

impl IEventWorker for EventWorker {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) {
        let selectors = self.selectors();
        let transformers = self.transformers();
        let output = Box::new(self.output.as_ref());
        let sink = SynchronizedEventSink { output, selectors, transformers };
        self.input.read(Box::new(&sink));
    }
}

impl EventWorker {
    fn selectors(&self) -> Vec<Box<&dyn IEventSelector>> {
        let mut selectors: Vec<Box<&dyn IEventSelector>> = Vec::new();
        for selector in &self.selectors {
            selectors.push(Box::new(selector.as_ref()));
        }
        selectors
    }

    fn transformers(&self) -> Vec<Box<&dyn IEventTransformer>> {
        let mut transformers: Vec<Box<&dyn IEventTransformer>> = Vec::new();
        for transformer in &self.transformers {
            transformers.push(Box::new(transformer.as_ref()));
        }
        transformers
    }
}

pub struct EventWorkerBuilder {
    name: String,
    input: Option<Box<dyn IEventInput>>,
    output: Option<Box<dyn IEventOutput>>,
    selectors: Vec<Box<dyn IEventSelector>>,
    transformers: Vec<Box<dyn IEventTransformer>>,
}

impl EventWorkerBuilder {
    pub fn new(name: &str) -> Self {
        EventWorkerBuilder {
            name: name.to_string(),
            input: None,
            output: None,
            selectors: Vec::new(),
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
        self.selectors.push(selector);
        self
    }

    pub fn add_transformer(&mut self, transformer: Box<dyn IEventTransformer>) -> &mut Self {
        self.transformers.push(transformer);
        self
    }

    pub fn build(&mut self) -> Box<dyn IEventWorker> {
        Box::new(EventWorker {
            name: self.name.clone(),
            input: self.input.take().expect("input is not set"),
            output: self.output.take().expect("output is not set"),
            selectors: self.selectors.drain(..).collect(),
            transformers: self.transformers.drain(..).collect(),
        })
    }
}