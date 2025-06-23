# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Substreams project that indexes wagering/betting events from a Solana smart contract (HS3 platform). It extracts and structures on-chain data about wagers being created and resolved.

Key details:
- **Target Program ID**: `6xw9f54fZrCFyhRtNBfA9tzuvwzUfe4DUw33JD1uyyfd`
- **Network**: Solana mainnet
- **Module Name**: `map_program_data`
- **Language**: Rust (compiled to WebAssembly)

## Essential Commands

### Building and Development
```bash
# Build the Substreams module (compiles Rust to WASM)
substreams build

# Launch GUI for visual debugging
substreams gui

# Run the module locally
substreams run -e <endpoint> substreams.yaml map_program_data

# Generate protobuf code
buf generate

# Format Rust code
cargo fmt

# Run linter
cargo clippy
```

### Development Environment Management
```bash
# Show all available dev commands
help

# Check development environment status
dev-status

# Restart all services (graph-node, postgres, ipfs)
dev-restart-services

# View graph-node logs
dev-logs-graphnode

# Open PostgreSQL shell
dev-psql
```

### Deployment
```bash
# Authenticate with StreamingFast
substreams auth

# Login to registry
substreams registry login

# Publish to registry
substreams registry publish

# Generate downstream sinks
substreams codegen subgraph
substreams codegen sql
```

## Architecture

### Core Data Flow
1. **Input**: Solana blocks from mainnet
2. **Filter**: Transactions for program ID `6xw9f54fZrCFyhRtNBfA9tzuvwzUfe4DUw33JD1uyyfd`
3. **Extract**: Both events (from logs) and instructions (from transaction data)
4. **Output**: Protocol buffer messages containing structured wager data

### Key Components

- **`src/lib.rs`**: Main module implementation
  - `map_program_data()`: Entry point that processes blocks
  - Event extraction: Parses `WagerInitialized` and `WagerResolved` events (V1 and V2)
  - Instruction extraction: Decodes `InitializeWager`, `ResolveWager`, and `FinalizeWagerV2` instructions

- **`proto/schema.proto`**: Protocol buffer definitions for output data structures
- **`generator.json`**: Anchor IDL defining the smart contract interface
- **`substreams.yaml`**: Module configuration and network settings

### Event/Instruction Types
The module tracks two versions of wager operations:
- **V1**: Original wager format
- **V2**: Enhanced format with additional fields (oracle_update_slot, finalize functionality)

Each wager operation has both an event (emitted in logs) and instruction (transaction data) representation that are extracted separately.

### Development Services
The devcontainer provides pre-configured services:
- PostgreSQL (port 5432): Stores subgraph data
- IPFS (port 5001): Content storage
- Graph Node (port 8000): GraphQL endpoint
- PGWeb (port 8081): Database UI