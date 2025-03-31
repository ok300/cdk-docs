use anyhow::Result;
use cdk::mint_url::MintUrl;
use cdk::nuts::{CurrencyUnit, Token};
use cdk::types::Melted;
use cdk::wallet::types::WalletKey;
use cdk::wallet::{MultiMintWallet, SendOptions};
use cdk::Amount;

#[allow(dead_code)]
// --8<-- [start:send]
pub async fn send(
    multi_mint_wallet: &MultiMintWallet,
    mint_url: MintUrl,
    send_amount: Amount,
    send_unit: CurrencyUnit,
) -> Result<Token> {
    let wallet_key = WalletKey::new(mint_url, send_unit.clone());
    let wallet = multi_mint_wallet.expect_wallet(&wallet_key).await?;

    let prepared_send = wallet
        .prepare_send(
            send_amount,
            SendOptions {
                include_fee: true,
                ..Default::default()
            },
        )
        .await?;
    println!("The fees are: {} {send_unit}", prepared_send.fee());

    let token = wallet.send(prepared_send, None).await?;
    println!("The token is: {token}");

    Ok(token)
}
// --8<-- [end:send]

#[allow(dead_code)]
// --8<-- [start:melt]
pub async fn melt(
    multi_mint_wallet: &MultiMintWallet,
    mint_url: MintUrl,
    send_unit: CurrencyUnit,
    bolt11: String,
) -> Result<Melted> {
    let wallet_key = WalletKey::new(mint_url, send_unit);
    let wallet = multi_mint_wallet.expect_wallet(&wallet_key).await?;

    let quote = wallet.melt_quote(bolt11, None).await?;
    let melt = wallet.melt(&quote.id).await?;

    Ok(melt)
}
// --8<-- [end:melt]
