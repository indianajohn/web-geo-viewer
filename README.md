web-geo-viewer
==============
This is a 3D renderer meant to be published to a static website, compiled in WASM. It
arose from my annoyance at the absence of a free, simple PLY viewer online.

See https://indianajohn.github.io/ for a live demo.

Dependencies
------------
wasmpack must be installed.

```
# You might have to run this once if you don't already have Nightly installed.
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install wasm-bindgen-cli
```
this crate only targets WASM, so cargo commands should be adjusted accordingly. For
example, if you wanted to run cargo fix, you should do this:

```
cargo fix --target=wasm32-unknown-unknown
```

Building
--------
Run `build.sh` to build.

Running
-------
Run 

```
./listen.sh
```

And then visit http://localhost:8000
