# Rusty ESP32

Playground for ESP32 with Rust
Generate via [esp-template](https://docs.esp-rs.org/esp-hal/esp-hal/0.17.0/esp32c3/esp_hal/)

## Getting started

- Be sure to have the following installed:
  -- clang
  -- libuv
  -- just
  -- e.g. run

```
yay -S clang libuv
```

- Install some cargo modules:

```
cargo install espflash ldproxy cargo-generate
```

- (Opt.) setup a new project:

```
cargo generate --git https://github.com/esp-rs/esp-template cargo
```

-- be sure to choose esp32c

- verify that you have the necessary access rights to device, e.g. add your self to the right group or create a udev rule

```
echo "SUBSYSTEMS==\"usb\", ATTRS{idVendor}==\"303a\", ATTRS{idProduct}==\"1001\", MODE=\"0660\", GROUP=\"plugdev\"" | sudo tee /etc/udev/rules.d/99-esp-rust-board.rules > /dev/null
sudo udevadm control --reload-rules && sudo udevadm trigger
```

## Flash&monitor

Just execute

```
just ui
cargo run
```

## References

- [Quick Start Guide Embassy and ESP32](https://dev.to/cyrilmarpaud/embedded-rust-on-esp32c3-board-a-hands-on-quickstart-guide-28mf)
- [Embassy repo](https://github.com/embassy-rs/embassy)
- [Embassy Docs](https://embassy.dev/dev/index.html)
- [EspFlash Docs](https://github.com/esp-rs/espflash/blob/main/cargo-espflash/README.md)

## Tools

- Saleae Gui for the logic analyzer, e.g. from the AUR:

```
yay -S saleae-logic2
```

- Gui for the digitial oscilliscope, e.g. from the AUR:

```
yay -S openhantek

```
