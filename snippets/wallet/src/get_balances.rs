use std::collections::BTreeMap;

use anyhow::Result;
use cdk::error::Error;
use cdk::wallet::multi_mint_wallet::MultiMintWallet;

#[allow(dead_code)]
// --8<-- [start:get_balances]
pub async fn mint_balances(multi_mint_wallet: &MultiMintWallet) -> Result<(), Error> {
    for wallet in multi_mint_wallet.get_wallets().await {
        let balance = wallet.total_balance().await?;
        let unit = wallet.unit;
        let mint_url = wallet.mint_url;

        println!("{mint_url}: {balance} {unit}");
    }

    Ok(())
}
// --8<-- [end:get_balances]
