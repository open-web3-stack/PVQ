.PHONY: run
run: chainspec
	bunx @acala-network/chopsticks@1.0.6 --config poc/runtime/chopsticks.yml --genesis output/chainspec.json

GUEST_EXAMPLES = $(shell find guest-examples -name "Cargo.toml" -not -path "guest-examples/Cargo.toml" | xargs -n1 dirname | xargs -n1 basename)
GUEST_TARGETS = $(patsubst %,guest-%,$(GUEST_EXAMPLES))
DUMMY_GUEST_TARGETS = $(patsubst %,dummy-guest-%,$(GUEST_EXAMPLES))

.PHONY: guests
guests: $(GUEST_TARGETS)

.PHONY: dummy-guests
dummy-guests: $(DUMMY_GUEST_TARGETS)

guest-%:
	cd guest-examples; METADATA_OUTPUT_DIR=$(realpath output) cargo build --release --bin guest-$* -p guest-$*
	mkdir -p output
	polkatool link --run-only-if-newer -s guest-examples/target/riscv32emac-unknown-none-polkavm/release/guest-$* -o output/guest-$*.polkavm

dummy-guest-%:
	mkdir -p output
	touch output/guest-$*.polkavm

.PHONY: tools
tools: polkatool chain-spec-builder pvq-program-metadata-gen

.PHONY: polkatool
polkatool:
	cargo install --path vendor/polkavm/tools/polkatool

.PHONY: pvq-program-metadata-gen
pvq-program-metadata-gen:
	cargo install --path pvq-program-metadata-gen

.PHONY: chain-spec-builder
chain-spec-builder:
	cargo install --locked chain-spec-builder@0.5.0

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: clippy
clippy:
	SKIP_WASM_BUILD= cargo clippy -- -D warnings
	cd guest-examples; METADATA_OUTPUT_DIR=$(realpath output) cargo clippy --all

.PHONY: test
test:
	SKIP_WASM_BUILD= cargo test

.PHONY: chainspec
chainspec:
	cargo build -p poc-runtime --release
	mkdir -p output
	cp target/release/wbuild/poc-runtime/poc_runtime.compact.compressed.wasm output
	chain-spec-builder -c output/chainspec.json create -n poc-runtime -i poc-runtime -r ./output/poc_runtime.compact.compressed.wasm -s default
	cat output/chainspec.json | jq '.properties = {}' > output/chainspec.json.tmp
	mv output/chainspec.json.tmp output/chainspec.json
