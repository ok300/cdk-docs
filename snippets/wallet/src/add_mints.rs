use std::str::FromStr;

use anyhow::Result;
use cdk::mint_url::MintUrl;
use cdk::nuts::CurrencyUnit;
use cdk::wallet::types::WalletKey;
use cdk::wallet::MintConnector;
use cdk::wallet::MultiMintWallet;
use cdk::HttpClient;

#[allow(dead_code)]
// --8<-- [start:add_mint]
async fn add_mint(multi_mint_wallet: &MultiMintWallet, mint_url_str: &str) -> Result<()> {
    let mint_url = MintUrl::from_str(mint_url_str)?;
    let client = HttpClient::new(mint_url.clone(), None);

    let mint_info = client.get_mint_info().await?;
    multi_mint_wallet
        .localstore
        .add_mint(mint_url.clone(), Some(mint_info))
        .await?;

    // TODO Show mint info to user, get confirmation

    let keys = client.get_mint_keys().await?;
    let units: Vec<CurrencyUnit> = keys.into_iter().map(|ks| ks.unit).collect();
    for unit in units {
        let wallet_key = WalletKey::new(mint_url.clone(), unit.clone());

        if !multi_mint_wallet.has(&wallet_key).await {
            multi_mint_wallet
                .create_and_add_wallet(mint_url_str, unit, None)
                .await?;
        }
    }

    Ok(())
}
// --8<-- [end:add_mint]
