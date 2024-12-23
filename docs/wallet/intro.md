# Intro

The CDK library helps applications integrate a Cashu wallet.

The `cdk-cli` is an example app built with the CDK. You may want to [give it a try](cli.md) to get familiar with the main concepts.

When you're ready, the following will show how to use the CDK to integrate a cashu wallet into your own application.

## Getting Started

Add the dependencies to your `Cargo.toml`:

```toml
cdk = { version = "v0.6.0", features = ["wallet"] }
cdk-sqlite = { version = "v0.6.0", features = ["wallet"] }
```

