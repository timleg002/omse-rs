# OBWS - The obws (obvious) remote control library for OBS

[![Build Status][build-img]][build-url]
[![Repository][crates-img]][crates-url]
[![Documentation][doc-img]][doc-url]
[![Code Coverage][cover-img]][cover-url]

[build-img]: https://img.shields.io/github/workflow/status/dnaka91/obws/CI/main?style=for-the-badge
[build-url]: https://github.com/dnaka91/obws/actions?query=workflow%3ACI
[crates-img]: https://img.shields.io/crates/v/obws?style=for-the-badge
[crates-url]: https://crates.io/crates/obws
[doc-img]: https://img.shields.io/badge/docs.rs-obws-4d76ae?style=for-the-badge
[doc-url]: https://docs.rs/obws
[cover-img]: https://img.shields.io/endpoint?url=https://dnaka91.github.io/obws/coverage.json&style=for-the-badge
[cover-url]: https://dnaka91.github.io/obws

Remote control OBS with the [obs-websocket] plugin from Rust 🦀.

[obs-websocket]: https://github.com/Palakis/obs-websocket

## V5 support

The upcoming obs-websocket v5 is already being worked on and most features are supported. Check out
the [v5-api] branch for more information on how to use it.

A new version will be released shortly after the relese of obs-websocket. Currently waiting on
missing docs and eventual breaking changes before the release.

[v5-api]: https://github.com/dnaka91/obws/tree/v5-api

## Usage

Add `obws` to your project with `cargo add obws` (needs [cargo-edit]) or add it manually to your
`Cargo.toml`:

```toml
[dependencies]
obws = "0.9.1"
```

In addition, you will need to use the latest [tokio](https://tokio.rs) runtime to use this library
as it makes heavy use of async/await and is bound to this runtime.

[cargo-edit]: https://github.com/killercup/cargo-edit

### Example

Here we connect to a OBS instance, get some version information and log in to access the whole API
and lastly print out a list of available scenes.

For more usage instructions see the [docs](https://docs.rs/obws) or check out the
[examples](examples/README.md).

```rust
use anyhow::Result;
use obws::Client;

#[tokio::main]
async fn main() -> Result<()> {
    /// Connect to the OBS instance through obs-websocket.
    let client = Client::connect("localhost", 4444).await?;

    /// Get and print out version information of OBS and obs-websocket.
    let version = client.general().get_version().await?;
    println!("{:#?}", version);

    /// Optionally log-in (if enabled in obs-websocket) to allow other APIs and receive events.
    client.login(Some("password")).await?;

    /// Get a list of available scenes and print them out.
    let scene_list = client.scenes().get_scene_list().await?;
    println!("{:#?}", scene_list);

    Ok(())
}
```

## License

This project is licensed under [MIT License](LICENSE) (or <http://opensource.org/licenses/MIT>).
