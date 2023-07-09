# matter_qrcode_generator

[![CI](https://github.com/thekuwayama/matter_qrcode_generator/workflows/CI/badge.svg)](https://github.com/thekuwayama/matter_qrcode_generator/actions?workflow=CI)
[![license](https://img.shields.io/badge/license-MIT/Apache2.0-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/matter_qrcode_generator/main/LICENSE-APACHE)
[![dependency status](https://deps.rs/repo/github/thekuwayama/matter_qrcode_generator/status.svg)](https://deps.rs/repo/github/thekuwayama/matter_qrcode_generator)

`matter_qrcode_generator` is QR-code generator for Matter.


```sh-session
$ matter_qrcode_generator 65521 32770 123456 250

█████████████████████████████████
█████████████████████████████████
████ ▄▄▄▄▄ █▄▄█▀ ▄▄ ██ ▄▄▄▄▄ ████
████ █   █ █▄ ▄▄▀▄█▀██ █   █ ████
████ █▄▄▄█ █▀ ▀██▀▀▄▄█ █▄▄▄█ ████
████▄▄▄▄▄▄▄█▄▀ ▀ ▀▄█▄█▄▄▄▄▄▄▄████
████▄▀▄█▀▀▄ █ █ █▄█▄▄▀▀ ▀█ ▀ ████
████  ▄▀▄▄▄▀▄▄▄  ▄ ▄▄█▄█▄ ▄▀ ████
█████▀ ▀█▄▄███▀▄█  ▄▄ █▄▄▄ ▀▀████
████▄▄█ ▄▄▄ ▄▄▄▄█▀█  █▄█▀▀  ▀████
████▄▄█▄▄█▄▄  ▀▄ ▄██ ▄▄▄  █▀▀████
████ ▄▄▄▄▄ █▄▄ ▀ ▄ ▄ █▄█ ▀█  ████
████ █   █ ██▀ ██   ▄▄▄▄ ▄█▄█████
████ █▄▄▄█ █▄▀▀▄█▀█▄▄█▄█  ▀▄ ████
████▄▄▄▄▄▄▄█▄█▄▄▄▄██▄█▄▄▄█▄▄▄████
█████████████████████████████████
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
```

## Install

You can install `matter_qrcode_generator` with the following:

```sh-session
$ cargo install --git https://github.com/thekuwayama/matter_qrcode_generator.git --branch main
```


## Usage

```sh-session
$ matter_qrcode_generator --help
QR-code generator for Matter

Usage: matter_qrcode_generator <VENDOR_ID> <PRODUCT_ID> <PASSCODE> <DISCRIMINATOR>

Arguments:
  <VENDOR_ID>      Vendor ID
  <PRODUCT_ID>     Product ID
  <PASSCODE>       passcode
  <DISCRIMINATOR>  discriminator

Options:
  -h, --help     Print help
  -V, --version  Print version
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/thekuwayama/matter_qrcode_generator/blob/main/LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/thekuwayama/matter_qrcode_generator/blob/main/LICENSE-MIT) or http://opensource.org/licenses/MIT)
