use anyhow::{anyhow, Result};
use cdk::mint_url::MintUrl;
use cdk::nuts::CurrencyUnit;
use cdk::wallet::types::{SendKind, WalletKey};
use cdk::wallet::MultiMintWallet;
use cdk::Amount;

#[allow(dead_code)]
// --8<-- [start:check_fees_before_swap]
pub async fn check_fees_before_swap(
    multi_mint_wallet: &MultiMintWallet,
    mint_url: MintUrl,
    send_amount: Amount,
    send_unit: CurrencyUnit,
) -> Result<()> {
    // TODO

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
// --8<-- [end:check_fees_before_swap]
