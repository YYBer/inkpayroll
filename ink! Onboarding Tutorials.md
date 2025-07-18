# Complete ink! Onboarding Tutorials

A comprehensive guide to getting started with ink! smart contracts on Polkadot.

## Table of Contents

1. [Introduction to ink!](#1-introduction-to-ink)
2. [Setting Up Your Development Environment](#2-setting-up-your-development-environment)
3. [Your First ink! Contract](#3-your-first-ink-contract)
4. [Understanding ink! Syntax](#4-understanding-ink-syntax)
5. [Building and Testing](#5-building-and-testing)
6. [Deployment Options](#6-deployment-options)
7. [Advanced Features](#7-advanced-features)
8. [Best Practices](#8-best-practices)
9. [Common Issues and Solutions](#9-common-issues-and-solutions)
10. [Resources and Next Steps](#10-resources-and-next-steps)

---

## 1. Introduction to ink!

### What is ink!?

ink! is Rust-based smart contract language for Polkadot and Substrate-based blockchains. It compiles to WebAssembly (Wasm) and runs on the `contracts` pallet.

### Key Features

- **Rust-based**: Leverage Rust's safety and performance
- **WebAssembly**: Compile to efficient Wasm bytecode
- **Polkadot native**: Built specifically for Polkadot ecosystem
- **Gas efficient**: Optimized for low transaction costs
- **Substrate compatible**: Works on any Substrate chain with contracts pallet

### Why Choose ink!?

- **Memory safety**: Rust prevents common bugs like buffer overflows
- **Performance**: WebAssembly provides near-native execution speed
- **Ecosystem**: Access to Rust's rich package ecosystem
- **Tooling**: Excellent development tools and IDE support

---

## 2. Setting Up Your Development Environment

### Prerequisites

- **Rust**: Version 1.70 or later
- **Node.js**: For front-end development (optional)
- **Docker**: For local blockchain node (recommended)

### Step 1: Install Rust

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your terminal, then verify
rustc --version
```

### Step 2: Install ink! Tools

```bash
# Install cargo-contract (ink! CLI tool)
cargo install cargo-contract --force

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Verify installation
cargo contract --version
```

### Step 3: Set Up Local Blockchain (Optional)

```bash
# Option 1: Using Docker (Recommended)
docker run --rm -p 9944:9944 paritytech/contracts-node:latest --dev --ws-external

# Option 2: Install locally
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
```

### Step 4: Install Browser Extension

1. Install [Polkadot.js Extension](https://polkadot.js.org/extension/)
2. Create a new account
3. Save your seed phrase securely

---

## 3. Your First ink! Contract

### Create a New Project

```bash
# Create a new ink! project
cargo contract new my_first_contract

# Navigate to the project
cd my_first_contract

# Check the project structure
ls -la
```

### Project Structure

```
my_first_contract/
├── Cargo.toml          # Project configuration
├── lib.rs              # Contract source code
└── .gitignore          # Git ignore file
```

### Basic Contract Template

```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod my_first_contract {
    
    #[ink(storage)]
    pub struct MyFirstContract {
        value: bool,
    }

    impl MyFirstContract {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let my_first_contract = MyFirstContract::default();
            assert_eq!(my_first_contract.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut my_first_contract = MyFirstContract::new(false);
            assert_eq!(my_first_contract.get(), false);
            my_first_contract.flip();
            assert_eq!(my_first_contract.get(), true);
        }
    }
}
```

### Build Your Contract

```bash
# Build the contract
cargo contract build

# You should see output like:
# Original wasm size: 45.8K, Optimized: 11.0K
# Your contract artifacts are ready. You can find them in:
# target/ink/my_first_contract.contract
```

---

## 4. Understanding ink! Syntax

### Contract Module

```rust
#[ink::contract]
mod my_contract {
    // Contract code goes here
}
```

### Storage

```rust
#[ink(storage)]
pub struct MyContract {
    value: u32,
    owner: AccountId,
    balances: Mapping<AccountId, Balance>,
}
```

### Constructors

```rust
#[ink(constructor)]
pub fn new(initial_value: u32) -> Self {
    Self { 
        value: initial_value,
        owner: Self::env().caller(),
        balances: Mapping::default(),
    }
}
```

### Messages (Functions)

```rust
// Read-only function
#[ink(message)]
pub fn get_value(&self) -> u32 {
    self.value
}

// Mutable function
#[ink(message)]
pub fn set_value(&mut self, new_value: u32) {
    self.value = new_value;
}

// Payable function
#[ink(message, payable)]
pub fn deposit(&mut self) {
    let amount = self.env().transferred_value();
    // Handle deposit logic
}
```

### Events

```rust
#[ink(event)]
pub struct ValueChanged {
    #[ink(topic)]
    old_value: u32,
    #[ink(topic)]
    new_value: u32,
}

// Emit event
self.env().emit_event(ValueChanged {
    old_value: old_val,
    new_value: new_val,
});
```

### Error Handling

```rust
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotOwner,
    InsufficientBalance,
    TransferFailed,
}

pub type Result<T> = core::result::Result<T, Error>;

#[ink(message)]
pub fn restricted_function(&mut self) -> Result<()> {
    if self.env().caller() != self.owner {
        return Err(Error::NotOwner);
    }
    // Function logic
    Ok(())
}
```

---

## 5. Building and Testing

### Building

```bash
# Build in debug mode
cargo contract build

# Build in release mode
cargo contract build --release

# Check for issues
cargo contract check
```

### Testing

```bash
# Run unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Testing Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn constructor_works() {
        let contract = MyContract::new(42);
        assert_eq!(contract.get_value(), 42);
    }

    #[ink::test]
    fn set_value_works() {
        let mut contract = MyContract::new(42);
        contract.set_value(100);
        assert_eq!(contract.get_value(), 100);
    }

    #[ink::test]
    fn only_owner_can_set_value() {
        let mut contract = MyContract::new(42);
        // This should work (called by default account)
        assert!(contract.restricted_function().is_ok());
    }
}
```

### End-to-End Testing

```bash
# Run E2E tests (requires running node)
cargo test --features e2e-tests
```

---

## 6. Deployment Options

### Local Development Node

```bash
# Start local node
contracts-node --dev --tmp

# Deploy contract
cargo contract instantiate \
  --constructor new \
  --args "42" \
  --suri //Alice \
  --url ws://127.0.0.1:9944
```

### Using Contracts UI

1. **Open Contracts UI**: https://contracts-ui.substrate.io
2. **Connect to Network**: `ws://127.0.0.1:9944`
3. **Upload Contract**: Select your `.contract` file
4. **Instantiate**: Choose constructor and parameters
5. **Interact**: Call functions and view results

### Testnet Deployment

```bash
# Deploy to testnet (example: Rococo Contracts)
cargo contract instantiate \
  --constructor new \
  --args "42" \
  --suri "your-seed-phrase" \
  --url wss://rococo-contracts-rpc.polkadot.io
```

---

## 7. Advanced Features

### Cross-Chain Messaging

```rust
#[ink(message)]
pub fn send_xcm(&mut self, dest: MultiLocation, message: Xcm) -> Result<()> {
    // XCM integration (advanced topic)
    Ok(())
}
```

### Upgradeable Contracts

```rust
#[ink(message)]
pub fn upgrade_contract(&mut self, new_code_hash: Hash) -> Result<()> {
    // Contract upgrade logic
    self.env().set_code_hash(&new_code_hash)?;
    Ok(())
}
```

### Oracle Integration

```rust
#[ink(message)]
pub fn get_price(&self) -> Result<u128> {
    // Oracle price feed integration
    Ok(42000) // Placeholder
}
```

---

## 8. Best Practices

### Security

- **Access Control**: Always check caller permissions
- **Input Validation**: Validate all function parameters
- **Overflow Protection**: Use safe arithmetic operations
- **Reentrancy**: Prevent reentrancy attacks

```rust
#[ink(message)]
pub fn secure_transfer(&mut self, to: AccountId, amount: Balance) -> Result<()> {
    // 1. Check conditions
    let caller = self.env().caller();
    let balance = self.balances.get(&caller).unwrap_or(0);
    
    if balance < amount {
        return Err(Error::InsufficientBalance);
    }
    
    // 2. Update state
    self.balances.insert(&caller, &(balance - amount));
    let to_balance = self.balances.get(&to).unwrap_or(0);
    self.balances.insert(&to, &(to_balance + amount));
    
    // 3. External calls (if any)
    // Always do external calls last
    
    Ok(())
}
```

### Gas Optimization

- **Minimize Storage**: Use efficient data structures
- **Batch Operations**: Combine multiple operations
- **Lazy Evaluation**: Compute values only when needed

### Testing

- **Unit Tests**: Test individual functions
- **Integration Tests**: Test contract interactions
- **E2E Tests**: Test full user workflows

---

## 9. Common Issues and Solutions

### Compilation Issues

**Problem**: `overflow-checks = true` error
```bash
# Solution: Add to Cargo.toml
[profile.release]
overflow-checks = false
```

**Problem**: Missing WebAssembly target
```bash
# Solution: Add target
rustup target add wasm32-unknown-unknown
```

### Deployment Issues

**Problem**: `CodeRejected` error
```bash
# Solution: Use compatible ink! version
# Check node version compatibility
```

**Problem**: `AccountUnmapped` error
```bash
# Solution: Map account first
# Use developer -> extrinsics -> revive -> mapAccount
```

### Runtime Issues

**Problem**: Out of gas
```bash
# Solution: Increase gas limit or optimize code
cargo contract instantiate --gas 1000000000
```

---

## 10. Resources and Next Steps

### Documentation

- **ink! Documentation**: https://use.ink/
- **Substrate Documentation**: https://docs.substrate.io/
- **Polkadot Documentation**: https://docs.polkadot.com/

### Tools

- **Contracts UI**: https://contracts-ui.substrate.io
- **ink! Playground**: https://ink-playground.substrate.io
- **Polkadot.js Apps**: https://polkadot.js.org/apps/

### Community

- **ink! GitHub**: https://github.com/paritytech/ink
- **Substrate Stack Exchange**: https://substrate.stackexchange.com/
- **Polkadot Discord**: https://discord.gg/polkadot

### Example Projects

- **ERC20 Token**: https://github.com/paritytech/ink-examples/tree/main/erc20
- **ERC721 NFT**: https://github.com/paritytech/ink-examples/tree/main/erc721
- **Multisig Wallet**: https://github.com/paritytech/ink-examples/tree/main/multisig

### Next Steps

1. **Build More Contracts**: Try different contract types
2. **Join the Community**: Participate in discussions
3. **Contribute**: Help improve ink! ecosystem
4. **Stay Updated**: Follow ink! development

---

## Conclusion

Congratulations! You've completed the ink! onboarding tutorial. You now have the knowledge to:

- Set up a complete ink! development environment
- Write, build, and test ink! smart contracts
- Deploy contracts to local and test networks
- Follow best practices for security and optimization
- Troubleshoot common issues

Start building your first production contract and join the growing ink! developer community!

---

*This tutorial is designed to be comprehensive yet beginner-friendly. For the most up-to-date information, always refer to the official ink! documentation.*