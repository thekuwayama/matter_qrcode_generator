# matter_qrcode_generator

[![CI](https://github.com/thekuwayama/matter_qrcode_generator/workflows/CI/badge.svg)](https://github.com/thekuwayama/matter_qrcode_generator/actions?workflow=CI)
[![license](https://img.shields.io/badge/license-MIT/Apache2.0-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/matter_qrcode_generator/main/LICENSE-APACHE)
[![dependency status](https://deps.rs/repo/github/thekuwayama/matter_qrcode_generator/status.svg)](https://deps.rs/repo/github/thekuwayama/matter_qrcode_generator)

`matter_qrcode_generator` is QR-code generator for Matter.

![cli-demo](docs/cli-demo.gif)


## Install

You can install `matter_qrcode_generator` with the following:

```sh-session
$ cargo install --git https://github.com/thekuwayama/matter_qrcode_generator.git --branch main
```


## Usage

```sh-session
$ matter_qrcode_generator --help
QR-code generator for Matter

Usage: matter_qrcode_generator [OPTIONS] <VENDOR_ID> <PRODUCT_ID> <PASSCODE> <DISCRIMINATOR>

Arguments:
  <VENDOR_ID>      Vendor ID
  <PRODUCT_ID>     Product ID
  <PASSCODE>       passcode
  <DISCRIMINATOR>  discriminator

Options:
  -f, --file <FILE>  path to the .png output file
  -h, --help         Print help
  -V, --version      Print version
```

For example:

```sh-session
$ matter_qrcode_generator 65521 32770 123456 250
```

```sh-session
$ matter_qrcode_generator 65521 32770 123456 250 --file output.png
```

## Web App with Wasm
`matter_qrcode_generator_wasm` is a Web Application to print QR-code using Wasm.

- https://thekuwayama.github.io/matter_qrcode_generator/


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/thekuwayama/matter_qrcode_generator/blob/main/LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/thekuwayama/matter_qrcode_generator/blob/main/LICENSE-MIT) or http://opensource.org/licenses/MIT)
