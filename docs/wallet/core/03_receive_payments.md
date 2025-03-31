# Receive payments

Receiving ecash tokens can be done in three ways:

- receiving ecash directly
- minting ecash tokens
- using Payment Requests ([NUT-18](https://cashubtc.github.io/nuts/18/))

## Receive ecash

In this case, the sender sends ecash:

```rust
--8<-- "snippets/wallet/src/receive_payments.rs:receive_token"
```


## Receive from Lightning

In this situation, the sender sends Lightning sats, and we receive ecash.

This is sometimes also referred to as "minting ecash".

```rust
--8<-- "snippets/wallet/src/receive_payments.rs:mint"
```

## Payment Request

See the [Payment Requests](../optional/01_payment_requests.md) section.