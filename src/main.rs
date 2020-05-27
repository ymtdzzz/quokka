extern crate quokka;

use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use quokka::*;

// GRCOV_EXCL_START
fn main() -> Result<()> {
    let config = Config::new(None)?;

    let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(&config.watch, RecursiveMode::Recursive)?;

    Ok(())
}
// GRCOV_EXCL_STOP
