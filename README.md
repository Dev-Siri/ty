# ty

> NOTE: This project is yet to be finished.

`ty` is an extremely small subset of `yt-dlp`, written entirely in Rust. Unlike `yt-dlp` and all the video-downloaders based around or on it, `ty` is meant to be minimal and provide a developer-facing API. It provides a heavily modular approach to extract video info, streams or the raw manifest from YouTube and a separate module for deciphering signatures.

The purpose of `ty` is not to be used as a CLI application or just as a Rust library, but to be ran on any platform, focused primarily on the client. It can be used in web-based projects through WebAssembly and in other languages, like Go or Swift, with its FFI bindings.

## Usage

Require the crate in your `Cargo.toml` file:

```toml
ty = "0.0.1"
```

Then use the crate in your Rust code:

```rs
use anyhow::Result;
// `Ty` is the public interface to fetch YouTube streams.
use ty::Ty;

async fn main() -> Result<()> {
  // Initialize `ty`
  let ty = Ty::new()?;

  // Now you can fetch depending on what you need.
  let manifest = ty.get_manifest(...).await?;
  let streams = ty.get_streams(...).await?;
  let video_info = ty.get_video_info(...).await?;

  Ok(())
}
```

### Managing Streams, Metadata and Manifests

TODO

## Developing Locally

Clone the repository.

```
$ git clone https://github.com/Dev-Siri/ty
```

## Credits

## License

This project is [MIT](LICENSE) licensed.
