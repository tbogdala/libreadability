# libreadability

A very thin Rust project that wraps readable's [readablilty.rs](https://github.com/readable-app/readability.rs) fork
of loyd's [readability.rs](https://github.com/loyd/readability.rs) which is derived from Mozilla's [readability.js](https://github.com/mozilla/readability) project.

It aims to provide the `readability.rs` library in compiled shared library format with a C compatible interface,
allowing it to be used with FFI from other programming languages.

# Build

The additional tweaks needed to make sure the shared library gets built are added to the `Cargo.toml` file already,
so all that should be needed is a simple cargo invocation.

```bash
cargo build --release
```

# License

MIT