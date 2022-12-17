# Mockoon rustify
## Table of Contents

- [Description](#description)
- [Feature](#feature)
- [Usage](#usage)
- [License](#license)
## Description
Simple server for generating http handlers from configurations with custom responses. 

## Feature
- APIs generated from api-config.json
- support jwt decoding.
- routing based on cid-country

## Usage
- clone repo
  - cargo build
  - cargo run
- executable
  - clone repo
  - cargo build
  - ./target/{profile}/mockoon-rust

## profiles
- debug
  - default
- release
  - cargo build --release

## License

&copy; 2022 Al-Ani Mohammed [mohammed.al-ani@infotamia.com](mohammed.al-ani@infotamia.com).

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](LICENSE-MIT))

at your option.

The [SPDX](https://spdx.dev) license identifier for this project is `MIT OR Apache-2.0`.