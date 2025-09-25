# Simple Multisig Squad - Hello World

This is a simple Anchor program that just says "Hello, World!" - perfect for learning and building step by step.

## What's Here

- **Simple Hello World Program**: Just logs a message when called
- **Basic CLI**: Command-line interface to interact with the program
- **Clean Dependencies**: Only essential Anchor dependencies

## Project Structure

```
multisig-squad/
├── src/
│   ├── lib.rs          # Smart contract (hello world program)
│   └── main.rs         # CLI client
├── Cargo.toml          # Dependencies
├── Anchor.toml         # Anchor configuration
└── README.md           # This file
```

## Quick Start

### 1. Build the Project

```bash
cargo build
```

### 2. Start Local Solana Validator

```bash
solana-test-validator
```

### 3. Run the Hello Command

```bash
cargo run hello
```

## Program Functions

### `hello`
- **Purpose**: Simple hello world function
- **Parameters**: None
- **Action**: Logs "Hello, World!" message
- **Accounts**: Just needs a user to sign the transaction

## CLI Commands

```bash
# Say hello
cargo run hello

# With custom RPC URL
cargo run -- --rpc-url https://api.devnet.solana.com hello

# With custom keypair
cargo run -- --keypair-path ~/.config/solana/devnet.json hello
```

## Next Steps

This is a simple starting point! You can build upon this by:

1. **Add More Functions**: Create new program functions
2. **Add State**: Store data in accounts
3. **Add Logic**: Implement actual multisig functionality
4. **Add Tests**: Write unit tests
5. **Deploy**: Deploy to devnet or mainnet

## Building Step by Step

1. ✅ **Hello World** - Simple function that logs a message
2. ⏳ **Counter** - Program that increments a number
3. ⏳ **Simple Vault** - Store some data
4. ⏳ **Basic Multisig** - Multiple signatures required
5. ⏳ **Advanced Multisig** - Time locks, proposal management

## Dependencies

- **Anchor**: Solana framework for smart contracts
- **Solana SDK**: Core Solana functionality
- **Tokio**: Async runtime for CLI
- **Clap**: Command-line argument parsing

## Configuration

The program ID is set in both `lib.rs` and `main.rs`:
```rust
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFp1Jg");
```

Make sure both files use the same program ID!
# multisig-squad
