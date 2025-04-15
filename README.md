# PVQ

PolkaVM Query for Polkadot

## Getting Started

### Prerequisites

- Pull vendored `polkavm` and patched (for XCM integration PoC) `polkadot-sdk`: `git submodule update --init --recursive`.
- Install `polkatool` (for relinking the standard RV32E ELF to a PolkaVM blob) and `chain-spec-builder` (for building chainspec from a wasm): `make tools`

### Run Examples

`guest-examples` contains several guest programs to test the PVQ.

1. Build guest program: `make guests`
2. Run test runner: `cargo run -p pvq-test-runner -- --program output/<guest-program>`

Available PoC guest programs:

- `guest-sum-balance`: sum the balances of multiple accounts
- `guest-total-supply`: get the total supply of an asset
- `guest-sum-balance-percent`: sum the balances of multiple accounts and calculate the percentage of the total supply

### RuntimeAPI PoC

1. Use chopsticks to start a local chain with the RuntimeAPI enabled: `make run`
2. Build guest programs: `make guests`
3. Run test runner to display hex-encoded `args` in tracing logs: `cargo run -p pvq-test-runner -- --program output/<guest-program>`
4. Upload `program` and `args` in PJS UI.

### XCM Integration PoC

The test case of XCM integration is located in `vendor/polkadot-sdk/polkadot/xcm/xcm-simulator/example/src/tests.rs`

```rust
#[test]
fn test_report_query() {
    ...
}
```
