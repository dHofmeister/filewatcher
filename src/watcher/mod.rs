use anyhow::Result;
use log::error;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;

/// Registers a file watcher for the given paths.
///
/// Takes a vector of `PathBuf` representing file paths and a channel sender
/// to send debounce event results. Returns a debouncer handle that implements
/// `Drop` to manage the watcher lifecycle.
pub async fn register_watcher(
    paths: Vec<PathBuf>,
    tx: mpsc::Sender<DebounceEventResult>,
) -> Result<impl Drop> {
    let handle = tokio::runtime::Handle::current();
    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        move |res: DebounceEventResult| {
            let tx = tx.clone();
            handle.spawn(async move {
                if let Err(e) = tx.send(res).await {
                    error!("Error sending event result: {:?}", e);
                }
            });
        },
    )
    .unwrap();

    for path in &paths {
        debouncer.watcher().watch(path, RecursiveMode::Recursive)?;
    }

    Ok(debouncer)
}
