use flow::flow::FlowBuilder;
use flow::readwrite::{ConsoleEventWriter, IterableEventReader};
use flow::selector::EventSelector;

fn main() {
    let mut flow = FlowBuilder::new("flow")
        .reader(Box::new(IterableEventReader::new(100)))
        .writer(Box::new(ConsoleEventWriter::new()))
        .build();
    flow.run();
}
