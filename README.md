# WIP:avesia - Infinite creation from a pencil.

`avesia` is a software for industry-level anime creation.

Visit [avesia.org](https://avesia.org/) for details.

## Warnings

- `avesia` is still under development.

## Goals

- **Productive**: Enables small-groups to create industry-standard animation.
- **Dynamic modeling**: Create 3D model from 2d illustration.

## Dependencies

- **task** (https://taskfile.dev/): task runner.

```bash
npm install -g @go-task/cli
```

- **rust** (https://rust-lang.org/): rust.

- **wasm-bindgen** (https://github.com/rustwasm/wasm-bindgen): wasm binding generator.

```bash
cargo install -f wasm-bindgen-cli
```

TODO: Linux package dependencies

## Build

```bash
task build
```

## License

AGPL-3.0-only
