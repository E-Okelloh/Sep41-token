# SEP-0041 Token Standard Implementation

A Soroban-based smart contract implementing the **SEP-0041 (Stellar Ecosystem Proposal 41)** token standard on the Stellar network. SEP-0041 defines a standardized interface for token contracts within the Soroban smart contract platform, enabling seamless interoperability for transfers, allowances, and balances.

##  Features

- **SEP-0041 Compliant**: Fully implements required token interfaces (name, symbol, decimals, balance, transfer, allowance, etc.).
- **Optimized for Soroban**: Written in Rust, leveraging Soroban's modern auth and storage features.
- **Testnet Ready**: Ready for deployment and integration tests.

## Prerequisite Setup

Ensure you have the modern Stellar development toolchain installed:
- Rust & Cargo (`wasm32-unknown-unknown` target)
- [Stellar CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup)

## Building the Contract

To compile the smart contract to WebAssembly (WASM):
\`\`\`bash
stellar contract build
\`\`\`
The compiled `.wasm` file will be generated in the `target/wasm32-unknown-unknown/release/` directory.

##  Deployment

1. **Fund your deployer identity** (e.g., `Juliet`) on the Testnet:
   \`\`\`bash
   stellar keys fund Juliet --network testnet
   \`\`\`

2. **Deploy the WASM bytecode**:
   \`\`\`bash
   stellar contract deploy \
     --wasm target/wasm32-unknown-unknown/release/sep41_token.wasm \
     --source Juliet \
     --network testnet
   \`\`\`

##  Running Tests

Execute the native Rust test suite:
\`\`\`bash
cargo test
\`\`\`
