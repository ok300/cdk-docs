# Intro

The CDK library helps applications integrate a Cashu wallet.

The `cdk-cli` is an example app built with the CDK. You may want to [give it a try](cli.md) to get familiar with the main concepts.

The following will show how to use the CDK to integrate a cashu wallet into your own application.

## Setup Project

Add the dependencies to your `Cargo.toml`:

```toml
--8<-- "snippets/wallet/Cargo.toml:cdk-deps"
```

## Initialize Singleton

```rust
--8<-- "snippets/wallet/src/main.rs:main"
```