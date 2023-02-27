use flow::flow::{BaseEventSelector, BaseEventSink, IterEventSource, BaseEventTransformer, BaseFlowBuilder};

fn main() {
    let mut flow = BaseFlowBuilder::new("flow")
        .source(Box::new(IterEventSource::new("source", 100)))
        .sink(Box::new(BaseEventSink::new("sink")))
        .add_selector(Box::new(BaseEventSelector::new("selector", 0)))
        .add_transformer(Box::new(BaseEventTransformer::new("transformer")))
        .build();
    flow.run();
}
