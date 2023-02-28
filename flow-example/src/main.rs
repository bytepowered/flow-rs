use flow::worker::EventWorkerBuilder;
use flow::readwrite::{ConsoleEventWriter, IterableEventReader};
use flow::selector::EventSelector;

fn main() {
    let mut worker = EventWorkerBuilder::new("flow")
        .input(Box::new(IterableEventReader::new(100)))
        .output(Box::new(ConsoleEventWriter::new()))
        .build();
    worker.run();
}
