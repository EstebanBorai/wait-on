<div>
  <h1 align="center">wait-on</h1>
  <h4 align="center">
    Library and CLI Utility to wait on the availability of resources such as Files, HTTP Servers, Ports & Sockets
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/wait-on.svg)](https://crates.io/crates/wait-on)
  [![Documentation](https://docs.rs/wait-on/badge.svg)](https://docs.rs/wait-on)
  ![Build](https://github.com/EstebanBorai/wait-on/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/wait-on/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/EstebanBorai/wait-on/workflows/fmt/badge.svg)

</div>

## Installation

```bash
cargo install wait-on
```

## Usage

### Wait for a file to exist or exit with code 1 if 10m elapses waiting

```bash
wait-on -t 10m file /path/to/file
```

### Wait for a HTTP Resource to respond

```bash
wait-on http GET https://example.com
```

### Wait for a Socket to be available using TCP Protocol

```bash
wait-on tcp -i 127.0.0.1 -p 8080
```

## License

This project is licensed under the MIT license and the Apache License 2.0.
