A trivial codebase that can be used to inspect the memory usage of wasmtime.

# Requirements

A recent version of rust, the `wasm32-unknown-unknown` target.

When using `rustup`, this can be achieved by doing:

```console
rustup target add wasm32-unknown-unknown
```

# Building

First of all, the guest code should be built:

```console
cd guest
make build
```

This will produce a `guest.wasm` file under the `guest` directory.

# Usage

Move do the `wasmtime-memory` directory and execute:

```console
cargo run --release
```

The program will produce this output:

```console
The wasm module said: 42
Taking 1h power nap
```

Now you can look into the memory areas assigned to the process:

```console
sudo pmap $(pgrep wasmtime)
```
