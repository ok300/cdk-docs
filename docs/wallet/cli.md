# CLI

The `cdk-cli` crate contains a simple wallet implementation.

It is not necessary for wallet development, but it can be a useful interactive exercise to get familiar with the main Cashu concepts and operations.

## Setup

Install it with:

```shell
cargo install cdk-cli
```

## Mint

> Minting is the operation by which cashu tokens are brought into existence (minted), typically by sending an equivalent amount of sats to the mint. See [NUT-04](https://cashubtc.github.io/nuts/04/).

Mint 10 test sats on a test mint with:

```shell
cdk-cli mint https://testnut.cashu.space 10
```

This creates a mint request for 10 sats and simulates the payment of a matching BOLT11 invoice. Since this is a test mint, these are only test sats for demonstration purposes. On a real mint, `melt` is the operation by which a user can convert real sats into ecash.

The end result is the CLI wallet should have the 10 sats as ecash, which can be seen by checking the balance:

```shell
cdk-cli balance
```

## Send ecash

Use

```shell
cdk-cli send
```

to send the 10 sats of ecash. A cashu token will be printed similar to

```
cashuBo2FteBtodHRwczovL3Rlc3RudXQuY2FzaHUuc3BhY2VhdWNzYXRhdIGiYWlIAJofKTJT5B5hcIKkYWEIYXN4QGIzNDRjZGViMDY0YjNiMWZjN2I2YzI5YzNmNDg1MTBlZWFkODIyMzMzZmQ2ZTBlZTY4ZmFhNjVmYTM3NzgzZDhhY1ghAgEhIRRixGTV5aA4C9w5z--YIUa8OQ53-NWODcC3IMIPYWT2pGFhAmFzeEA2MzgyODE4NmQ3YjRiOTI0YmEyMmNlMGIyYWQ4MGZlY2ZkZTc2N2I2ZDlmNWQ0NDQzNzkxMjRjZmE0YmI5Nzc1YWNYIQIjJX-SeR6Ct5dp5mYP9INA62BbhQhKVP5Mjn05qNxjFmFk9g==
```

Checking the balance after `send` will show a reduced balance.

The ecash can be sent by simply sending this ecash token to the recipient via any communications medium (QR code, email, etc).

The ecash token itself contains all the information necessary for a cashu wallet to claim the corresponding sats from that mint.

To dig deeper and see the actual contents of the token, use

```
cdk-cli decode-token cashuB...
```

and read [NUT-00](https://cashubtc.github.io/nuts/00/) for the meaning of each field.


## Receive ecash

Any wallet (including the sender) can receive the token:

```shell
cdk-cli receive cashuB...
```

which will then reflect in the wallet's balance.

## Melt

> Melting is the reverse of minting. It's the operation by which Cashu tokens can be redeemed for the artefact they represent, typically a sat amount. See [NUT-05](https://cashubtc.github.io/nuts/05/).

A wallet that holds an ecash balance can melt ecash with

```shell
cdk-cli melt
```

Since this example used fake sats from a test mint, the BOLT11 invoice will not be paid by the mint. However, when using a real mint, `melt` would allow the receiver to convert the ecash back into sats.
