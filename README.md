# Foresta Contracts

Foresta Contracts is a set of smart contracts designed to enable a decentralized marketplace for scientific algorithms. It leverages RMRK's NFT standard, Phala Network's secure computation capabilities, and IPFS for decentralized storage.

## Purpose Overview

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
- `lib.rs`: Implements the schrödinger contract, including functions encrypting and decrypting assets from IPFS CID's  within the `phala-contract` folder.

### 🏗️ How to use - Contracts

#### 💫 Build

- Use this [instructions](https://use.ink/getting-started/setup) to setup your ink!/Rust environment

##### Build the contracts

```sh
cd proxy
cargo +nightly contract build --release
# Default toolchain is nightly 1.80, update rust-toolchain if you encounter issues.
#channel = "nightly-2024-04-28"
```

```sh
- After running these commands, the contract artifacts (JSON file, Wasm file, and contract file) will be generated in the respective target/ink/ directories of each contract.
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

##### 1. Build the contracts your local node (Foresta Node Fork)

```sh
cd proxy
cargo +nightly contract build --release
# Default toolchain is nightly 1.80, update rust-toolchain if you encounter issues.
#channel = "nightly-2024-04-28"
```

```sh
- After running these commands, the contract artifacts (JSON file, Wasm file, and contract file) will be generated in the respective target/ink/ directories of each contract.
```

##### 2. Run the node with the contracts pallet or deploy to deployed

```sh
- Ensure the node is configured to include the contracts pallet.
- Start your node with the appropriate configuration.
```

##### 3. Upload and instantiate the contracts using Polkadot.js Apps

```sh
- Open a web browser and navigate to https://polkadot.js.org/apps.
- Connect to your local node by selecting the appropriate endpoint in the top left corner.
- Go to the "Developer" tab and click on "Contracts".
For each contract:

- Click on the "Upload & Instantiate Contract" button.
- In the "Upload New Contract Code" section, click on the "Upload" button and select the .contract file generated in step 1 for the respective contract.
- Provide the necessary constructor parameters (if any) in the "Deploy" section.
- Click on the "Deploy" button to instantiate the contract.

- Repeat the above steps for each contract you want to deploy.
```

##### 4. Interact with the deployed contracts

```sh
- Once the contracts are deployed, you can interact with them using the Polkadot.js Apps interface.
- In the "Contracts" section, you will see the list of deployed contracts.
- Click on a contract to expand its details and view the available methods.
- You can execute the methods by providing the necessary parameters and clicking on the "Execute" button.
```

##### 💫 Deployed contracts

Test on Galápagos Testnet on Tanssi - [TBD](https:////TBD)
