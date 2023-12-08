# Rusty ESP32
Playground for ESP32 with Rust


## Getting started
- Install some cargo modules:
```
cargo install espflash ldproxy cargo-generate
```
- (Opt.) setup a new project:
```
cargo generate --git https://github.com/esp-rs/esp-idf-template cargo
```
-- be sure to chooes esp32c3

## References
- [Quick Start Guide Embassy and ESP32](https://dev.to/cyrilmarpaud/embedded-rust-on-esp32c3-board-a-hands-on-quickstart-guide-28mf)
- [Embassy repo](https://github.com/embassy-rs/embassy)
- [Embassy Docs](https://embassy.dev/dev/index.html)

## Tools

- Gui for the logic analyzer, e.g. from the AUR:
```
yay -S logicanalyzer-git
```
- Gui for the digitial oscilliscope, e.g. from the AUR:
```
yay -S openhantek

```
