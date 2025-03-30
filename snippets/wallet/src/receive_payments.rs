use std::str::FromStr;

use anyhow::Result;
use cdk::amount::SplitTarget;
use cdk::mint_url::MintUrl;
use cdk::nuts::MintQuoteState;
use cdk::nuts::NotificationPayload;
use cdk::nuts::SecretKey;
use cdk::nuts::Token;
use cdk::nuts::{CurrencyUnit, ProofsMethods};
use cdk::wallet::types::WalletKey;
use cdk::wallet::MultiMintWallet;
use cdk::Amount;
use cdk::WalletSubscription;

#[allow(dead_code)]
// --8<-- [start:receive_token]
async fn receive_token(
    multi_mint_wallet: &MultiMintWallet,
    token_str: &str,
    signing_keys: &[SecretKey],
    preimage: &[String],
) -> Result<Amount, anyhow::Error> {
    let token: Token = Token::from_str(token_str)?;

    let mint_url = token.mint_url()?;
    let unit = token.unit().unwrap_or_default();
    let wallet_key = WalletKey::new(mint_url.clone(), unit.clone());

    if !multi_mint_wallet.has(&wallet_key).await {
        multi_mint_wallet
            .create_and_add_wallet(&mint_url.to_string(), unit, None)
            .await?;
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
    mint_url: MintUrl,
    amount: Amount,
    unit: CurrencyUnit,
    description: Option<String>,
) -> Result<()> {
    let wallet_key = WalletKey::new(mint_url.clone(), unit.clone());
    let wallet = match multi_mint_wallet.get_wallet(&wallet_key).await {
        Some(wallet) => wallet.clone(),
        None => {
            multi_mint_wallet
                .create_and_add_wallet(&mint_url.to_string(), unit.clone(), None)
                .await?
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
