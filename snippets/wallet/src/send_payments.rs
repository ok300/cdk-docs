use anyhow::{anyhow, Result};
use cdk::mint_url::MintUrl;
use cdk::nuts::CurrencyUnit;
use cdk::wallet::types::{SendKind, WalletKey};
use cdk::wallet::MultiMintWallet;
use cdk::Amount;

#[allow(dead_code)]
// --8<-- [start:send]
pub async fn send(
    multi_mint_wallet: &MultiMintWallet,
    mint_url: MintUrl,
    send_amount: Amount,
    send_unit: CurrencyUnit,
) -> Result<()> {
    let wallet_key = WalletKey::new(mint_url, send_unit);
    let token = multi_mint_wallet
        .send(
            &wallet_key,
            send_amount,
            None,
            None,
            SendKind::OnlineExact,
            true,
        )
        .await?;
    println!("{}", token);

    Ok(())
}
// --8<-- [end:send]

#[allow(dead_code)]
// --8<-- [start:melt]
pub async fn melt(
    multi_mint_wallet: &MultiMintWallet,
    mint_url: MintUrl,
    send_unit: CurrencyUnit,
    bolt11: String,
) -> Result<()> {
    let wallet_key = WalletKey::new(mint_url, send_unit);
    let wallet = multi_mint_wallet
        .get_wallet(&wallet_key)
        .await
        .ok_or(anyhow!("Unknown wallet"))?;

    let quote = wallet.melt_quote(bolt11, None).await?;
    let melt = wallet.melt(&quote.id).await?;

    Ok(())
}
// --8<-- [end:melt]
