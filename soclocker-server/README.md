# SocLocker Server

SocLocker Client implements the server side of the SocLocker reference
implementation.

Architecturally, it is built using [Rocket](https://rocket.rs/) as an 
underlying http server framework, with [Rust](https://www.rust-lang.org/) as
it's core implementation language.

The server is designed to be served from the same host as the client, with the
server being interacted with on `https://host.name/api/` and client being
interacted with on `https://host.name/`.

## Building

First, you must configure your servers private and public key. This can be done
by altering the `Rocket.toml` program

```
cargo build --release
```

This will create the executable, `./target/release/soclocker-server` which must
be run with a `Rocket.toml` 