# Validator Delegations Snapshot Tool

This Rust project fetches validator delegations from a Cosmos-based blockchain (specifically Coreum, but should work with any Cosmos SDK chain) using gRPC and exports the data to a CSV file.

## Features

- Connects to a Coreum full node using gRPC
- Fetches validator delegations for a specific validator
- Exports delegation data to a CSV file
- Error handling using custom `AppError` enum
- Asynchronous operations with Tokio runtime

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/elix1er/rustgrpc-cosmos-delegations.git
   cd rustgrpc-cosmos-delegations
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

Run the compiled binary:

```bash
./target/release/rustgrpc-cosmos-delegations
```

Or - just run `cargo run` to build and run the project.

The program will:

1. Connect to the Coreum full node
2. Fetch validator delegations
3. Create a CSV file in the `output` directory named `validator_delegations.csv`

## Configuration

The validator address is currently hardcoded in the `fetch_validator_delegations` function.

To change the validator address, modify this line before compiling.

## Project Structure

- `src/main.rs`: Contains the main logic for fetching delegations and creating the CSV file

## Dependencies

- `tonic`: gRPC client
- `tokio`: Asynchronous runtime
- `cosmos-sdk-proto`: Cosmos SDK protobuf definitions
- `csv`: CSV file creation
- `thiserror`: Error handling

## Testing

Run the tests using:

```bash
cargo test
```

---
The `test_fetch_validator_delegations` function in `src/tests.rs` tests the main functionality of fetching validator delegations.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
