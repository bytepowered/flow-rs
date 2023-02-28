pub trait IEvent {
    fn id(&self) -> u64;
    fn source(&self) -> &str;
    fn tag(&self) -> u64;
    fn timestamp(&self) -> u64;
    fn kind(&self) -> u16;
}

pub trait IEventBuilder {
    fn id(&mut self, id: u64) -> &mut Self;
    fn source(&mut self, source: &str) -> &mut Self;
    fn tag(&mut self, tag: u64) -> &mut Self;
    fn timestamp(&mut self, timestamp: u64) -> &mut Self;
    fn kind(&mut self, kind: u16) -> &mut Self;
    fn build(&self) -> Box<dyn IEvent>;
}

pub trait IEventReader {
    fn read<'a>(&self, sink: Box<&'a dyn IEventSink>);
}

pub trait IEventSink {
    fn next(&self, event: Box<dyn IEvent>);
}

pub trait IEventSelector {
    fn select(&self, event: Box<&dyn IEvent>) -> bool;
}

pub trait IEventWriter {
    fn write(&self, event: Box<dyn IEvent>);
}

pub trait IEventTransformer {
    fn transform(&self, event: Box<dyn IEvent>) -> Option<Box<dyn IEvent>>;
}

pub trait IFlow {
    fn name(&self) -> &str;
    fn run(&self);
}

pub trait FlowBuilder {
    fn name(&mut self, name: &str) -> &mut Self;
    fn source(&mut self, source: Box<dyn IEventReader>) -> &mut Self;
    fn sink(&mut self, sink: Box<dyn IEventSink>) -> &mut Self;
    fn add_selector(&mut self, selector: Box<dyn IEventSelector>) -> &mut Self;
    fn add_transformer(&mut self, transform: Box<dyn IEventTransformer>) -> &mut Self;
    fn build(&self) -> Box<dyn IFlow>;
}