# Send payments

!!! info "Automatic swap"
    Sending will automatically `swap` (see [NUT-03](https://cashubtc.github.io/nuts/03/)) tokens if necessary, to ensure the correct amount is sent.

## Send ecash

```rust
--8<-- "snippets/wallet/src/send_payments.rs:send"
```

## Send Lightning

```rust
--8<-- "snippets/wallet/src/send_payments.rs:melt"
```
