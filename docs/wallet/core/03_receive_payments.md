# Receive payments

Receiving ecash tokens can be done in a few ways:

- receiving ecash directly
- minting ecash tokens

## Receive ecash

In this case, the sender sends ecash and we receive ecash:

```rust
--8<-- "snippets/wallet/src/receive_payments.rs:receive_token"
```


## Receive from Lightning

In this situation, the sender sends Lightning sats, for which the mint creates ecash, and we receive the ecash.

This is sometimes also referred to as "minting ecash".

```rust
--8<-- "snippets/wallet/src/receive_payments.rs:mint"
```
