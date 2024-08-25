# FileWatcher
[![Filewatcher](https://github.com/dHofmeister/filewatcher/actions/workflows/rust.yml/badge.svg)](https://github.com/dHofmeister/filewatcher/actions/workflows/rust.yml)

A cross-platform file system monitor.

## Research

The file system monitor has two core components:

    User Interface Library
    Monitoring Library

### File Monitor

The most common implementation of file monitoring is achieved by wrapping around OS native file system monitors:

    Linux: inotify subsystem
    Windows: Native API
    MacOS: FSEvents

Luckily, there already exists such a wrapper in Rust, which provides a single API that can handle all three operating systems: [notify](https://docs.rs/notify/latest/notify/)

The documentation mentions edge cases, one of them regarding debouncing. Therefore, it was chosen to use the [notify_debouncer_mini](https://docs.rs/notify-debouncer-mini/latest/notify_debouncer_mini/) 
### Command Line Interface

There are two de facto standards in the CLI Rust world:
    [Clap](https://docs.rs/clap/latest/clap/) and
    [Ratatui](https://docs.rs/ratatui/0.23.0/ratatui/)
    
For simplicity, Clap will be used in this project.
### Error Handling

Rust has the anyhow crate and the thiserror crate. Commonly, thiserror is used for libraries and anyhow for applications. This software is defined as an application, thus anyhow is used.

### Logging

A simple [env_logger](https://crates.io/crates/env_logger) is used where it was deemed appropriate.

### Future Enhancements
- Ratatui: Could be a nice interface for such a system; however, it would probably require more time to develop, and its benefits might not be too great.
- Auto-diff: An output that immediately shows changes could be useful.

## Usage
```bash
cargo run -- --help
```

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/filewatcher --help`
File System Monitor

Usage: filewatcher [OPTIONS]

Options:
  -p, --paths <PATHS>...
  -h, --help              Print help
  -V, --version           Print version
```

For example:
```
RUST_LOG=INFO cargo run -- -p '.'
```
```
[2024-08-25T20:30:49Z INFO  filewatcher::paths] Full path: /home/dev/repos/filewatcher
```
Changes in the mentioned folders will how be reflected in the logs:
```
[2024-08-25T20:32:19Z INFO  filewatcher] Change detected: "/home/dev/repos/filewatcher/new_file"
```
Multiple directories are supported, if paths contain spaces, then supply them in quotes:
```
RUST_LOG=DEBUG cargo run -- -p './data/directory with spaces' './data/directory'
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/filewatcher -p './data/directory with spaces' ./data/directory`
[2024-08-25T20:33:54Z INFO  filewatcher::paths] Full path: /home/dev/repos/filewatcher/data/directory with spaces
[2024-08-25T20:33:54Z INFO  filewatcher::paths] Full path: /home/dev/repos/filewatcher/data/directory
```

## Tests
Basic tests are implemented:
```
cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/filewatcher-1ad5b90083b4c8be)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/filewatcher-e29fbb9d692c9a5d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/test_changes.rs (target/debug/deps/test_changes-1ffb76c8dfaf9cf4)

running 1 test
test test_file_change_detection ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.21s

     Running tests/test_reading.rs (target/debug/deps/test_reading-6300f2732a7f2d91)

running 4 tests
test test_multiple_paths ... ok
test test_spaces_complex ... ok
test test_spaces_simple ... ok
test test_valid_input ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests filewatcher

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
