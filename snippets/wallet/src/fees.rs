use anyhow::Result;
use cdk::mint_url::MintUrl;
use cdk::nuts::CurrencyUnit;
use cdk::wallet::types::WalletKey;
use cdk::wallet::{MultiMintWallet, SendOptions};
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
    let token = wallet.send(prepared_send, None).await?;
    println!("{}", token);

    Ok(())
}
// --8<-- [end:check_fees_before_swap]
