mod add_mints;
mod get_balances;
mod receive_payments;
mod send_payments;

use std::sync::Arc;

use anyhow::Result;
use bip39::Mnemonic;
use cdk::cdk_database::WalletDatabase;
use cdk::nuts::CurrencyUnit;
use cdk::wallet::MultiMintWallet;
use cdk_sqlite::WalletSqliteDatabase;

fn main() {}

#[allow(dead_code)]
// --8<-- [start:init_wallet]
async fn init_wallet() -> Result<Arc<MultiMintWallet>> {
    let work_dir = std::env::current_dir()?;
    let sql_path = work_dir.join("cdk-db.sqlite");
    let localstore = Arc::new(WalletSqliteDatabase::new(&sql_path).await?);
    let seed = Arc::new(Mnemonic::generate(12)?.to_seed_normalized(""));

    let multi_mint_wallet = MultiMintWallet::new(localstore.clone(), seed, vec![]);
    for (mint_url, _) in localstore.get_mints().await? {
        multi_mint_wallet
            .create_and_add_wallet(&mint_url.to_string(), CurrencyUnit::Sat, None)
            .await?;
    }

    Ok(Arc::new(multi_mint_wallet))
}
// --8<-- [end:init_wallet]
