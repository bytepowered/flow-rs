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
    fn read(&self, sink: Box<dyn Sink>);
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
    fn transform(&mut self, event: Box<dyn Event>) -> Option<Box<dyn Event>>;
}

pub trait Flow {
    fn name(&self) -> &str;
    fn run(&mut self);
}

pub trait FlowBuilder {
    fn name(&mut self, name: &str) -> &mut Self;
    fn source(&mut self, source: Box<dyn Source>) -> &mut Self;
    fn sink(&mut self, sink: Box<dyn Sink>) -> &mut Self;
    fn add_filter(&mut self, filter: Box<dyn Filter>) -> &mut Self;
    fn add_transformer(&mut self, transform: Box<dyn Transformer>) -> &mut Self;
    fn build(&self) -> Box<dyn Flow>;
}