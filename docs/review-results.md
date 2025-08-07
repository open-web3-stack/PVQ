# Code Review Results

This document contains the results of the code review.

## General Findings

*   [Add general findings here]

### `pvq-executor`

- **No README.md**: The crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The crate is missing tests.

### `pvq-extension-swap`

- **No README.md**: The crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The crate is missing tests.

### `pvq-extension`

- **No README.md**: The crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The crate is missing tests.

### `pvq-primitives`

- **No README.md**: The crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The crate is missing tests.
- **Undocumented Enum**: The `PvqError` enum variants should be documented.

### `pvq-program-metadata-gen`

- **No Tests**: The crate is missing tests.
- **Use of `expect()`**: The binary uses `expect()` which can cause panics. It should be replaced with proper error handling.
- **Long `main` function**: The `main` function in `src/bin/pvq-program-metadata-gen.rs` should be broken down into smaller, more manageable functions.

### `pvq-program`

- **No README.md**: The crate and its procedural macro sub-crate should have `README.md` files.
- **No Tests**: The crate and its procedural macro sub-crate are missing tests. UI tests for the procedural macro are particularly important.

### `pvq-runtime-api`

- **No README.md**: The crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The crate is missing tests.
- **Inefficient Data Types**: The API uses `Vec<u8>` for parameters and return types where slices (`&[u8]`) could be more efficient.

### `pvq-test-runner`

- **No README.md**: The crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The test runner itself is not tested.
- **Use of `expect()`/`unwrap()`**: The crate uses `expect()` and `unwrap()` extensively, which can lead to panics.
- **Long `main` function**: The `main` function in the binary is too long and should be refactored.
- **Hardcoded test data**: The test data and expected results are hardcoded within the runner. It would be more flexible to load them from external files (e.g., JSON or YAML).

### `poc/runtime`

- **No README.md**: The runtime crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The runtime and its extension implementations are not tested.
- **Hardcoded Gas Limit**: The default gas limit in the `execute_query` runtime API is hardcoded. It should be a constant.

### `guest-examples`

- **No README.md**: The `guest-examples` directory should have a `README.md` explaining each example.
- **Magic Numbers**: The examples use hardcoded `extension_id`s. These should be replaced with named constants.
- **Lack of Comments**: The examples could benefit from more comments explaining the code.
- **Misleading Calculation**: The `sum_balance_percent` example has a potentially misleading percentage calculation that should be clarified. Unit tests should be added for the executor logic and error handling.
- **`unwrap()` in `PvqExecutor::new`**: The `unwrap()` call should be replaced with proper error handling.
- **Long `execute` function**: The `execute` function in `executor.rs` should be broken down into smaller, more manageable functions.
- **Tuple as return type**: The return type of the `execute` function should be a struct for better readability.
- **Lack of comments**: More comments could be added to `executor.rs` to explain the execution flow.

### `pvq-extension-core`

- **No README.md**: The crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The crate is missing tests.
- **Commented-out functions**: The `lib.rs` file contains commented-out functions that should be implemented or removed.

### `pvq-extension-fungibles`

- **No README.md**: The crate should have a `README.md` explaining its purpose and how to use it.
- **No Tests**: The crate is missing tests.
