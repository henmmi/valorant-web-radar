# Setup and Installation

- [Rust and Cargo](https://rustup.rs/)
- [Node.js and npm](https://nodejs.org/)

```
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-pack
$ npm install
$ cargo install wasm-bindgen-cli
```

# Build
```
$ cd canvas
$ cargo build
$ npm run serve
```

and then visiting http://localhost:8080 in a browser should run !
