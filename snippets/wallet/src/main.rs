use std::sync::Arc;

use cdk::Amount;
use cdk::Wallet;
use cdk::cdk_database::WalletMemoryDatabase;
use cdk::nuts::CurrencyUnit;
use rand::Rng;

// --8<-- [start:main]
#[tokio::main]
async fn main() {
    let seed = rand::thread_rng().gen::<[u8; 32]>();

    let mint_url = "https://testnut.cashu.space";
    let unit = CurrencyUnit::Sat;
    let amount = Amount::from(10);
    aaa

    let localstore = WalletMemoryDatabase::default();

    let wallet = Wallet::new(mint_url, unit, Arc::new(localstore), &seed, None).unwrap();
}
// --8<-- [end:main]
