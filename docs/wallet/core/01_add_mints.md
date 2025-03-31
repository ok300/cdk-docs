# Add Mints

!!! info "Trust model"
    Adding a mint to the CDK `MultiMintWallet` implies the user trusts this mint enough to hold a cashu balance on it. Adding a mint should therefore need explicit user confirmation in the UI.


```rust
--8<-- "snippets/wallet/src/add_mints.rs:add_mint"
```

The `MintInfo` is described in [NUT-06](https://cashubtc.github.io/nuts/06/). 