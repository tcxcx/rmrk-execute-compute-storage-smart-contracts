# Foresta Contracts

Foresta Contracts is a set of smart contracts designed to enable a decentralized marketplace for scientific algorithms. It leverages RMRK's NFT standard, Phala Network's secure computation capabilities, and IPFS for decentralized storage.

### Purpose Overview
The Foresta ecosystem revolves around two main types of NFTs:

1. **Algorithm Storage NFTs (AS-NFTs)**: These NFTs represent the ownership of an encrypted algorithm stored on IPFS. The owner of an AS-NFT has control over the algorithm's content and can update it.

2. **Execution Access NFTs (EA-NFTs)**: These NFTs grant the holder the right to execute the associated algorithm within a secure computational environment provided by Phala Network. EA-NFTs are minted under a parent AS-NFT.

### License
Apache 2.0

## Contract Structure

The Foresta Contracts are organized into the following modules:

- `lib.rs`: The main library file that combines all the modules within `algo_execute` folder.
- `proxy.rs`: Contains the main `RmrkProxy` contract implementation, including functions for minting AS-NFTs and EA-NFTs, executing algorithms, and interacting with the catalog contract within the `proxy` folder.
- `types.rs`: Defines the type definitions and error enums used across the contract within the `proxy` folder.
- `lib.rs`: Implements the catalog contract, including functions for adding and retrieving assets  within the `catalog` folder.


### 🏗️ How to use - Contracts
##### 💫 Build
- Use this [instructions](https://use.ink/getting-started/setup) to setup your ink!/Rust environment

```sh
cd proxy
cargo contract build --release
```

##### 💫 Run unit and integration tests

```sh
cd proxy
cargo test --features e2e-tests -- --nocapture
```
##### 💫 Deploy
First start your local node. Recommended is the latest [swanky-node](https://github.com/AstarNetwork/swanky-node/releases)
```sh
./target/release/swanky-node --dev --tmp -lruntime=trace -lruntime::contracts=debug -lerror
```
Use
- polkadot.JS. Instructions on [Astar docs](https://docs.astar.network/docs/build/wasm/tooling/polkadotjs)
- or [Contracts UI](https://contracts-ui.substrate.io/)

to deploy contract on the local Swanky node

##### 💫 Deployed contracts
Test on Galápagos Testnet on Tanssi - [TBD](https:////TBD)
