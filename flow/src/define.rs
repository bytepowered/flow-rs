use async_trait::async_trait;

pub trait IEvent: Send + Sync {
    fn id(&self) -> u64;
    fn source(&self) -> &str;
    fn tag(&self) -> u64;
    fn timestamp(&self) -> u64;
    fn kind(&self) -> u16;
}

#[async_trait]
pub trait IEventInput  : Send + Sync {
    async fn read<'a>(&self, sink: Box<&'a dyn IEventSink>) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait IEventSink : Send + Sync {
    async fn next(&self, event: Box<dyn IEvent>) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait IEventOutput : Send + Sync  {
    async fn write(&self, event: Box<dyn IEvent>) -> Result<(), anyhow::Error>;
}

pub trait IEventSelector: Sync {
    fn select(&self, event: Box<&dyn IEvent>) -> bool;
}

pub trait IEventTransformer: Sync {
    fn transform(&self, event: Box<&dyn IEvent>) -> Option<Box<dyn IEvent>>;
}

#[async_trait]
pub trait IEventWorker : Send + Sync {
    fn name(&self) -> &str;
    async fn run(&self) -> Result<(), anyhow::Error>;
}

pub trait FlowBuilder {
    fn name(&mut self, name: &str) -> &mut Self;
    fn source(&mut self, source: Box<dyn IEventInput>) -> &mut Self;
    fn sink(&mut self, sink: Box<dyn IEventSink>) -> &mut Self;
    fn add_selector(&mut self, selector: Box<dyn IEventSelector>) -> &mut Self;
    fn add_transformer(&mut self, transform: Box<dyn IEventTransformer>) -> &mut Self;
    fn build(&self) -> Box<dyn IEventWorker>;
}