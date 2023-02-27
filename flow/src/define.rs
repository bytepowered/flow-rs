pub trait Event {
    fn id(&self) -> u64;
    fn source(&self) -> &str;
    fn tag(&self) -> u64;
    fn timestamp(&self) -> u64;
    fn kind(&self) -> u16;
}

pub trait EventBuilder {
    fn id(&mut self, id: u64) -> &mut Self;
    fn source(&mut self, source: &str) -> &mut Self;
    fn tag(&mut self, tag: u64) -> &mut Self;
    fn timestamp(&mut self, timestamp: u64) -> &mut Self;
    fn kind(&mut self, kind: u16) -> &mut Self;
    fn build(&self) -> Box<dyn Event>;
}

pub trait EventSource {
    fn name(&self) -> &str;
    fn read(&self, sink: Box<dyn EventSink>);
}

pub trait EventSink {
    fn next(&mut self, event: Box<dyn Event>);
}

pub trait EventSelector {
    fn name(&self) -> &str;
    fn select(&self, event: Box<dyn Event>) -> bool;
}

pub trait EventTransformer {
    fn name(&self) -> &str;
    fn transform(&self, event: Box<dyn Event>) -> Option<Box<dyn Event>>;
}

pub trait Flow {
    fn name(&self) -> &str;
    fn run(&mut self);
}

pub trait FlowBuilder {
    fn name(&mut self, name: &str) -> &mut Self;
    fn source(&mut self, source: Box<dyn EventSource>) -> &mut Self;
    fn sink(&mut self, sink: Box<dyn EventSink>) -> &mut Self;
    fn add_selector(&mut self, selector: Box<dyn EventSelector>) -> &mut Self;
    fn add_transformer(&mut self, transform: Box<dyn EventTransformer>) -> &mut Self;
    fn build(&self) -> Box<dyn Flow>;
}