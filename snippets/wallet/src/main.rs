mod add_mints;
mod get_balances;
mod receive_payments;
mod send_payments;

use std::sync::Arc;

use anyhow::Result;
use bip39::Mnemonic;
use cdk::cdk_database::WalletDatabase;
use cdk::wallet::MultiMintWallet;
use cdk::Wallet;
use cdk_sqlite::WalletSqliteDatabase;

fn main() {}

#[allow(dead_code)]
// --8<-- [start:init_wallet]
async fn init_wallet() -> Result<Arc<MultiMintWallet>> {
    let work_dir = std::env::current_dir()?;
    let sql_path = work_dir.join("cdk-db.sqlite");
    let localstore = WalletSqliteDatabase::new(&sql_path).await.map(Arc::new)?;

    let seed = Mnemonic::generate(12)?.to_seed_normalized("");

    let mut wallets: Vec<Wallet> = Vec::new();
    let mints = localstore.get_mints().await?;
    for (mint_url, _) in mints {
        let wallet = Wallet::new(
            &mint_url.to_string(),
            cdk::nuts::CurrencyUnit::Sat,
            localstore.clone(),
            &seed,
            None,
        )?;

        wallets.push(wallet);
    }
    let multi_mint_wallet = MultiMintWallet::new(wallets);

    Ok(Arc::new(multi_mint_wallet))
}
// --8<-- [end:init_wallet]
