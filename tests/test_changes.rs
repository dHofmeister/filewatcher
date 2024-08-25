use tempfile::NamedTempFile;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_file_change_detection() {
    env_logger::init();

    // INFO: Create a temporary file to trigger file watcher
    log::debug!("Creating temp file");
    let test_file = NamedTempFile::new_in(".").expect("Failed to create temp file");
    let test_file_path = test_file.path().to_path_buf();

    tokio::fs::write(&test_file_path, b"Hello, RocSys!")
        .await
        .expect("Failed to write to temp file");

    let mut child = Command::new("./target/debug/filewatcher")
        .arg("-p")
        .arg(test_file_path.parent().unwrap().to_str().unwrap())
        .spawn()
        .expect("Failed to spawn filewatcher");
    log::debug!("Spawned executable");

    // INFO: Read

    sleep(Duration::from_millis(101)).await;
    tokio::fs::write(&test_file_path, b"Modifying...")
        .await
        .expect("Failed to write to temp file");
    sleep(Duration::from_millis(101)).await;
    log::debug!("Modified File");

    child.kill().await.expect("Failed to kill child process");

    sleep(Duration::from_secs(1)).await;

    // NOTE: Unfinished test
    // Should capture stdout and assert the change to be detected
    // for now, it has to be manually confirmed by reading the logs
}
