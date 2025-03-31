# Send payments

!!! info "Automatic swap"
    Sending will automatically `swap` (see [NUT-03](https://cashubtc.github.io/nuts/03/)) tokens if necessary, to ensure the correct amount is sent.

## Send ecash

```rust
--8<-- "snippets/wallet/src/send_payments.rs:send"
```

## Send to Lightning

In this situation, the sender converts ecash into Lightning sats and pays the receiver's Lightning invoice in one step.

This is sometimes also referred to as "melting ecash".

```rust
--8<-- "snippets/wallet/src/send_payments.rs:melt"
```
