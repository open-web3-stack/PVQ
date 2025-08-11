# PVQ Test Runner

CLI to execute PVQ guest programs against the extensions and executor used in this repository.

### Use cases

- Prepare input bytes for bundled example programs
- Execute a program and print the raw result
- Inspect generated extension metadata

### CLI usage

```bash
cargo run -p pvq-test-runner -- --program path/to/program.polkavm --chain poc --entrypoint-idx 0

# Print only prepared input/expected result
cargo run -p pvq-test-runner -- --program path/to/program.polkavm --chain poc --entrypoint-idx 0 --print-data
```

### Build

```bash
cargo build -p pvq-test-runner --release
```
