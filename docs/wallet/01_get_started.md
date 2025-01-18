# Get Started

## Setup Project

CDK supports several storage backends:

- `cdk-redb` for Redb
- `cdk-rexie` for Rexie
- `cdk-sqlite` for SQLite

Add the dependencies for CDK and the chosen storage backend to your `Cargo.toml`:

```toml
--8<-- "snippets/wallet/Cargo.toml:cdk-deps"
```

## Initialize CDK Wallet

Based on the chosen DB type, initialize the DB location and the CDK wallet:

```rust
--8<-- "snippets/wallet/src/main.rs:init_wallet"
```

