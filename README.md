# filewatcher
cross-platform file system monitor


## Research
---
The file system monitor has two core components, 1. The user interface, 2. The monitoring library. Explained below:


### File monitor
The most common implementation of file monitoring is achieved by wrapping around OS native file system monitors, for linux it's the inotify subsystem. https://man7.org/linux/man-pages/man7/inotify.7.html, for windows it's the native API and for MacOS it's FSEvents.

Luckily there already exists such a wrapper in rust, which provide a single API that can handle all three operating systems: https://docs.rs/notify/latest/notify/

The documentation mentions edge cases, one of them regarding debouncing. Therefore it was chosen to use the notify_debouncer_mini crate: https://docs.rs/notify-debouncer-mini/latest/notify_debouncer_mini/

### Command Line Interface
There are two defacto standards in the CLI rust world, Clap: https://docs.rs/clap/latest/clap/ and Ratatui: https://github.com/ratatui/Ratatui. For simplicity, Clap will be used in this project. 

### Error handling
Rust has the anyhow crate and the thiserror crate. Commonly, thiserror is used for libraries and anyhow for applications. This software is defined as an application, thus anyhow is used.

### Future...
Ratatui could be a nice interface for such a system however it would probably require more time to develop and its benefits might not be too great.
Tab auto-complete, for CLI based software, especially ones that are expected to handle file paths, tab auto-completes makes the user's interaction better, a possible crate for this feature: https://crates.io/crates/clap_complete
