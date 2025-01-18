use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use cdk::cdk_database;
use cdk::cdk_database::WalletDatabase;
use cdk::mint_url::MintUrl;
use cdk::nuts::CurrencyUnit;
use cdk::wallet::client::MintConnector;
use cdk::wallet::multi_mint_wallet::WalletKey;
use cdk::wallet::MultiMintWallet;
use cdk::{HttpClient, Wallet};

#[allow(dead_code)]
// --8<-- [start:add_mint]
async fn add_mint(
    multi_mint_wallet: &MultiMintWallet,
    localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Send + Sync>,
    seed: &[u8],
    mint_url_str: &str,
) -> Result<()> {
    let mint_url = MintUrl::from_str(mint_url_str)?;
    let client = HttpClient::new(mint_url.clone());

    let mint_info = client.get_mint_info().await?;

    // TODO Show mint info to user, get confirmation

    let keys = client.get_mint_keys().await?;
    let units: Vec<CurrencyUnit> = keys.into_iter().map(|ks| ks.unit).collect();
    for unit in units {
        let wallet_key = WalletKey::new(mint_url.clone(), unit.clone());

        if multi_mint_wallet.get_wallet(&wallet_key).await.is_none() {
            let wallet = Wallet::new(mint_url_str, unit, localstore.clone(), seed, None)?;
            multi_mint_wallet.add_wallet(wallet).await;
        }
    }

    Ok(())
}
// --8<-- [end:add_mint]
