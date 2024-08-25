# FileWatcher

A cross-platform file system monitor.

## Research

The file system monitor has two core components:

    User Interface
    Monitoring Library

### File Monitor

The most common implementation of file monitoring is achieved by wrapping around OS native file system monitors:

    Linux: inotify subsystem (inotify man page)
    Windows: Native API
    MacOS: FSEvents

Luckily, there already exists such a wrapper in Rust, which provides a single API that can handle all three operating systems: notify crate.

The documentation mentions edge cases, one of them regarding debouncing. Therefore, it was chosen to use the notify_debouncer_mini crate: notify_debouncer_mini crate.
### Command Line Interface

There are two de facto standards in the CLI Rust world:

    Clap: Clap crate
    Ratatui: Ratatui GitHub

For simplicity, Clap will be used in this project.
### Error Handling

Rust has the anyhow crate and the thiserror crate. Commonly, thiserror is used for libraries and anyhow for applications. This software is defined as an application, thus anyhow is used.
Future Enhancements

    Ratatui: Could be a nice interface for such a system; however, it would probably require more time to develop, and its benefits might not be too great.
    Tab Auto-Complete: For CLI-based software, especially ones that are expected to handle file paths, tab auto-completes make the user's interaction better. A possible crate for this feature: clap_complete.

## Summary

This README outlines the core components and libraries used for building a cross-platform file system monitor in Rust. The notify crate is used for file monitoring, notify_debouncer_mini for debouncing events, and clap for the command-line interface. Error handling is managed using the anyhow crate. Future enhancements may include a Ratatui interface and tab auto-complete functionality.
