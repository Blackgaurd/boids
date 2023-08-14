# Boids

Boids simulation written in rust and javascript.

## Running

First, install wasm-pack:

```bash
cargo install wasm-pack
```

Install and build:

```bash
git clone https://github.com/Blackgaurd/boids.git
cd boids
wasm-pack build --target web --release
```

Run the application:

```bash
python3 -m http.server
```
