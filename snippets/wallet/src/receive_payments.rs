use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use cdk::amount::SplitTarget;
use cdk::cdk_database;
use cdk::cdk_database::Error;
use cdk::cdk_database::WalletDatabase;
use cdk::mint_url::MintUrl;
use cdk::nuts::MintQuoteState;
use cdk::nuts::NotificationPayload;
use cdk::nuts::SecretKey;
use cdk::nuts::Token;
use cdk::nuts::{CurrencyUnit, ProofsMethods};
use cdk::wallet::types::WalletKey;
use cdk::wallet::MultiMintWallet;
use cdk::Amount;
use cdk::Wallet;
use cdk::WalletSubscription;

#[allow(dead_code)]
// --8<-- [start:receive_token]
async fn receive_token(
    multi_mint_wallet: &MultiMintWallet,
    localstore: Arc<dyn WalletDatabase<Err = cdk_database::Error> + Send + Sync>,
    seed: &[u8],
    token_str: &str,
    signing_keys: &[SecretKey],
    preimage: &[String],
) -> Result<Amount, anyhow::Error> {
    let token: Token = Token::from_str(token_str)?;

    let mint_url = token.mint_url()?;

    let wallet_key = WalletKey::new(mint_url.clone(), token.unit().unwrap_or_default());

    if multi_mint_wallet.get_wallet(&wallet_key).await.is_none() {
        let wallet = Wallet::new(
            &mint_url.to_string(),
            token.unit().unwrap_or_default(),
            localstore,
            seed,
            None,
        )?;
        multi_mint_wallet.add_wallet(wallet).await;
    }

    let amount = multi_mint_wallet
        .receive(token_str, signing_keys, preimage)
        .await?;
    Ok(amount)
}
// --8<-- [end:receive_token]

#[allow(dead_code)]
// --8<-- [start:mint]
pub async fn mint(
    multi_mint_wallet: &MultiMintWallet,
    seed: &[u8],
    localstore: Arc<dyn WalletDatabase<Err = Error> + Sync + Send>,
    mint_url: MintUrl,
    amount: Amount,
    unit: CurrencyUnit,
    description: Option<String>,
) -> Result<()> {
    let wallet = match multi_mint_wallet
        .get_wallet(&WalletKey::new(mint_url.clone(), unit.clone()))
        .await
    {
        Some(wallet) => wallet.clone(),
        None => {
            let wallet = Wallet::new(&mint_url.to_string(), unit, localstore, seed, None)?;

            multi_mint_wallet.add_wallet(wallet.clone()).await;
            wallet
        }
    };

    let quote = wallet.mint_quote(amount, description).await?;

    println!("Quote: {:#?}", quote);

    println!("Please pay: {}", quote.request);

    let mut subscription = wallet
        .subscribe(WalletSubscription::Bolt11MintQuoteState(vec![quote
            .id
            .clone()]))
        .await;

    while let Some(msg) = subscription.recv().await {
        if let NotificationPayload::MintQuoteBolt11Response(response) = msg {
            if response.state == MintQuoteState::Paid {
                break;
            }
        }
    }

    let receive_proofs = wallet.mint(&quote.id, SplitTarget::default(), None).await?;
    let receive_amount = receive_proofs.total_amount()?;

    println!("Received {receive_amount} from mint {mint_url}");

    Ok(())
}
// --8<-- [end:mint]
