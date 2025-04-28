use config::Config;
use processor::ProcessorSignal;
use signal_hook::{consts::{SIGINT, SIGTERM}, iterator::Signals};

mod group;
mod domain;
mod rule;
mod domain_store;
mod orig_cache;
mod config;
mod processor;
mod module;

fn main() {
    let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();

    let config = Config::from_args();

    let processor = processor::start(config);

    for sig in signals.forever() {
        match sig {
            SIGINT | SIGTERM => {
                processor.send(ProcessorSignal::Stop);
                processor.join();
                break;
            },
            _ => {}
        }
    }
}
