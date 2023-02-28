use flow::worker::EventWorkerBuilder;
use flow::readwrite::{ConsoleEventWriter, IterableEventReader};
use flow::selector::EventSelector;

fn main() {
    let mut worker = EventWorkerBuilder::new("flow")
        .reader(Box::new(IterableEventReader::new(100)))
        .writer(Box::new(ConsoleEventWriter::new()))
        .build();
    worker.run();
}
