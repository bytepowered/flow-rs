use flow::flow::{EventSelector, EventSink, IterableEventSource, BaseEventTransformer, FlowBuilder};

fn main() {
    let mut flow = FlowBuilder::new("flow")
        .reader(Box::new(IterableEventSource::new("source", 100)))
        .sink(Box::new(EventSink::new("sink")))
        .add_selector(Box::new(EventSelector::new("selector", 0)))
        .add_transformer(Box::new(BaseEventTransformer::new("transformer")))
        .build();
    flow.run();
}
