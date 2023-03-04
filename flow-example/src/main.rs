use flow::readwrite::{ConsoleEventWriter, IterableEventReader};
use flow::selector::EventSelector;
use flow::worker::EventWorkerBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut worker = EventWorkerBuilder::new("flow")
        .input(Box::new(IterableEventReader::new(100)))
        .output(Box::new(ConsoleEventWriter::new()))
        .build();
    worker.run().await?;
    Ok(())
}
