# Bascoin Node Deployment Manual

This manual provides instructions for deploying and configuring Bascoin nodes, including generating custom genesis blocks and setting up multi-node environments.

## Table of Contents

1.  [Introduction](#1-introduction)
2.  [Prerequisites](#2-prerequisites)
3.  [Building Bascoin from Source](#3-building-bascoin-from-source)
4.  [Generating Custom Genesis Blocks](#4-generating-custom-genesis-blocks)
    *   [Understanding Genesis Blocks](#understanding-genesis-blocks)
    *   [Mining a New Genesis Block](#mining-a-new-genesis-block)
5.  [Running a Bascoin Node](#5-running-a-bascoin-node)
    *   [Basic Configuration](#basic-configuration)
    *   [Network Types (Mainnet, Testnet, Simnet, Devnet, Krippy, Nippy, Biero)](#network-types-mainnet-testnet-simnet-devnet-krippy-nippy-biero)
    *   [Configuring DNS Seeders](#configuring-dns-seeders)
6.  [Multi-Node Setup](#6-multi-node-setup)
    *   [Local Multi-Node Setup](#local-multi-node-setup)
    *   [Distributed Multi-Node Setup](#distributed-multi-node-setup)
7.  [Troubleshooting](#7-troubleshooting)

## 1. Introduction

Bascoin is a high-throughput, DAG-based cryptocurrency. This guide aims to help users deploy their own Bascoin nodes for various purposes, including development, testing, and contributing to the network.

## 2. Prerequisites

Before you begin, ensure you have the following installed:

*   **Rust and Cargo**: Follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install `rustup`, which includes `cargo`.
*   **Git**: For cloning the Bascoin repository.

## 3. Building Bascoin from Source

To build the Bascoin binaries from source, follow these steps:

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/your-repo/rusty-bascoin.git
    cd rusty-bascoin
    ```
    (Note: Replace `your-repo` with the actual Bascoin repository owner/name if it's different)

2.  **Build the project**:
    ```bash
    cargo build --release
    ```
    This will compile all necessary binaries, including `bascoind` (the full node daemon) and `bascoin-cli`.

## 4. Generating Custom Genesis Blocks

### Understanding Genesis Blocks

The genesis block is the first block in a blockchain or DAG, serving as the common ancestor for all subsequent blocks. For Bascoin, you can generate custom genesis blocks for development or private network setups. This is particularly useful for testing different network parameters or starting a fresh network without relying on the public mainnet/testnet.

### Mining a New Genesis Block

Bascoin provides a utility to mine custom genesis blocks. This process can be resource-intensive, especially for networks with high difficulty. For development and testing purposes (e.g., Krippy, Nippy, Biero), the genesis blocks are configured with very low difficulty, making them quick to mine.

To mine a genesis block, navigate to the project root and run the `bascoin-genesis-miner` utility. This utility will output the `hash`, `nonce`, and `timestamp` for each defined genesis block (Mainnet, Testnet, Simnet, Devnet, Krippy, Nippy, and Biero).

```bash
$env:RUST_LOG="info"; cargo run --release --package bascoin-genesis-miner > genesis_output.txt 2>&1
```

After running the command, inspect `genesis_output.txt` for the generated parameters. You will see output similar to this (values will vary based on your system and the time of generation):

```
[2025-08-20T04:25:52Z INFO  bascoin_genesis_miner] Mining genesis block for network: [..., bascoin-mainnet]
[2025-08-20T04:25:52Z INFO  bascoin_genesis_miner]   hash: Hash::from_bytes([0xa2, 0x20, 0xd, ..., 0x0, 0x0, 0x0]),
[2025-08-20T04:25:52Z INFO  bascoin_genesis_miner]   nonce: 0x109e4db3,
[2025-08-20T04:25:52Z INFO  bascoin_genesis_miner]   timestamp: 1755663749252,

[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner] Mining genesis block for network: [..., bascoin-krippy]
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   hash: Hash::from_bytes([0x88, 0xde, 0x23, ..., 0xd3, 0xb8, 0x49]),
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   nonce: 0x1,
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   timestamp: 1755663956556,

[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner] Mining genesis block for network: [..., bascoin-nippy]
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   hash: Hash::from_bytes([0x88, 0xde, 0x23, ..., 0xd3, 0xb8, 0x49]),
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   nonce: 0x1,
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   timestamp: 1755663956556,

[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner] Mining genesis block for network: [..., bascoin-biero]
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   hash: Hash::from_bytes([0x88, 0xde, 0x23, ..., 0xd3, 0xb8, 0x49]),
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   nonce: 0x1,
[2025-08-20T04:25:56Z INFO  bascoin_genesis_miner]   timestamp: 1755663956556,
```

You will need to copy these generated values and update the corresponding `hash`, `nonce`, and `timestamp` fields in `consensus/core/src/config/genesis.rs` for each network. For `Krippy`, `Nippy`, and `Biero`, these values might remain `0x1` for nonce and the same hash due to the high bits value (low difficulty).

## 5. Running a Bascoin Node

### Basic Configuration

To run a Bascoin node, you can use the `bascoind` executable found in `target/release/` after building. By default, it will attempt to connect to the Mainnet. You can specify the network using the `--net` flag.

```bash
# Run a Mainnet node
./target/release/bascoind --net mainnet

# Run a Testnet node
./target/release/bascoind --net testnet

# Run a Simnet node (typically used for local development/testing)
./target/release/bascoind --net simnet

# Run a Devnet node
./target/release/bascoind --net devnet

# Run a Krippy node
./target/release/bascoind --net krippy

# Run a Nippy node
./target/release/bascoind --net nippy

# Run a Biero node
./target/release/bascoind --net biero
```

### Network Types (Mainnet, Testnet, Simnet, Devnet, Krippy, Nippy, Biero)

Bascoin supports several network types, each with its own genesis block and parameters:

*   **Mainnet**: The primary Bascoin network.
*   **Testnet**: A network for testing new features without affecting the main chain.
*   **Simnet**: A simulated network for local development and testing, where proof-of-work can be skipped.
*   **Devnet**: A development network.
*   **Krippy**: A custom network with very low difficulty for quick genesis mining and testing.
*   **Nippy**: Another custom network with very low difficulty.
*   **Biero**: Another custom network with very low difficulty.

### Configuring DNS Seeders

DNS seeders are used by nodes to discover other peers on the network. You can configure custom DNS seeders by modifying the `dns_seeders` array in the `Params` struct for each network in `consensus/core/src/config/params.rs`.

For example, to add custom seeders for Mainnet:

```rust
pub const MAINNET_PARAMS: Params = Params {
    dns_seeders: &[
        "your-mainnet-seeder-1.example.com",
        "your-mainnet-seeder-2.example.com",
        // ... existing seeders ...
    ],
    // ... other params ...
};
```

## 6. Multi-Node Setup

### Local Multi-Node Setup

For local development and testing, you can run multiple Bascoin nodes on the same machine. This is typically done using the `simnet` or newly created custom networks (Krippy, Nippy, Biero) where PoW is easier to handle or skipped.

1.  **Run the first node (e.g., simnet)**:
    ```bash
    ./target/release/bascoind --net simnet --listen=127.0.0.1:18111 --rpclisten=127.0.0.1:18112
    ```

2.  **Run additional nodes, connecting to the first node**: Ensure each node uses unique P2P and RPC ports and connects to the first node's P2P address.

    ```bash
    ./target/release/bascoind --net simnet --listen=127.0.0.1:18211 --rpclisten=127.0.0.1:18212 --addpeer=127.0.0.1:18111
    ```

    Repeat for more nodes, incrementing port numbers and adding peers as needed.

### Distributed Multi-Node Setup

For a distributed setup across multiple machines, ensure your nodes have public IP addresses or are accessible within your network. You will need to configure the `--listen` and `--addpeer` flags with the appropriate public IPs.

1.  **Configure `bascoind` on each machine**:

    On Machine 1 (Public IP: `X.X.X.X`):
    ```bash
    ./target/release/bascoind --net mainnet --listen=X.X.X.X:18111
    ```

    On Machine 2 (Public IP: `Y.Y.Y.Y`):
    ```bash
    ./target/release/bascoind --net mainnet --listen=Y.Y.Y.Y:18111 --addpeer=X.X.X.X:18111
    ```

    And so on for other machines. Remember to open the necessary ports in your firewall.

## 7. Troubleshooting

*   **Compilation Errors**: If you encounter compilation errors after making changes, ensure you have run `cargo clean` and then `cargo build --release` again. Also, double-check that all `kaspa` references have been correctly replaced with `bascoin`.
*   **Node Not Connecting**: Verify firewall settings, ensure correct IP addresses and ports are used, and check the node's logs for connection errors (`RUST_LOG=info bascoind --net mainnet`).
*   **Genesis Block Issues**: If your custom genesis block is not being recognized, ensure its `hash`, `nonce`, and `timestamp` are accurately updated in `consensus/core/src/config/genesis.rs` and `consensus/core/src/config/params.rs`.
