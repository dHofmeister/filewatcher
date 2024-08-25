use anyhow::Result;
use env_logger;
use tokio::sync::mpsc;

use filewatcher::{parse, paths, watcher};

/// Initializes the logger, parses command-line arguments, canonicalizes input paths,
/// registers a file watcher, and enters the main event loop to handle file change events.
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = parse::parse_args();
    let (tx, mut rx) = mpsc::channel(1);

    // INFO: Read Input Paths
    let full_paths = paths::canonicalize_paths(&args.paths)?;

    // INFO: Register File Watcher
    let _debouncer_handle = watcher::register_watcher(full_paths, tx).await?;

    // INFO: Main Event Loop
    while let Some(events) = rx.recv().await {
        match events {
            Ok(events) => {
                for event in events {
                    log::info!("Change detected: {:?}", event.path);
                }
            }
            Err(e) => log::error!("Error: {:?}", e),
        }
    }

    Ok(())
}
