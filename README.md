# Valorant Web Radar

Display valorant agent locations on a 2D map, inspired by Leetify 2D Replay (CSGO). Written in Rust, compiles to wasm using wasm-bindgen to build JavaScript packages in Rust.

# Preview with simulated data
![image](https://github.com/henmmi/valorant-web-radar/assets/69184583/03ace6d3-4dd8-40b9-a3b9-800b4aae5a9c)

# Setup and Installation

- [Rust and Cargo](https://rustup.rs/)
- [Node.js and npm](https://nodejs.org/)

```
$ cargo build
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-pack
$ npm install
```

# Canvas Build
```
$ cd canvas
$ cargo build
$ npm run serve
```

and then visiting http://localhost:8080 in a browser should run !


# Server Build
```
$ cargo run --bin server
```

# Test-Client Build
```
$ cargo run --bin test-client
```
