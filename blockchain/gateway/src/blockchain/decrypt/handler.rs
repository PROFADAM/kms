use crate::blockchain::blockchain_impl;
use crate::blockchain::ciphertext_provider::CiphertextProvider;
use crate::blockchain::decrypt::handler::k256::ecdsa::SigningKey;
use crate::common::provider::GatewayContract;
use crate::config::EthereumConfig;
use crate::config::GatewayConfig;
use crate::events::manager::DecryptionEvent;
use crate::util::wallet::WalletManager;
use ethers::abi::encode;
use ethers::abi::Token;
use ethers::prelude::*;
use events::kms::FheType;
use std::sync::Arc;

#[retrying::retry(stop=(attempts(5)|duration(20)),wait=fixed(5))]
pub(crate) async fn handle_event_decryption(
    client: &Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
    event: &Arc<DecryptionEvent>,
    config: &GatewayConfig,
) -> anyhow::Result<()> {
    tracing::debug!("🍻 handle_event_decryption enter");
    let mut tokens: Vec<Token> = Vec::with_capacity(event.filter.cts.len());
    let client_clone = Arc::clone(client);
    let event = Arc::clone(event);
    let conf = config.clone();
    let _handle = tokio::spawn(async move {
        let ethereum_wss_url = conf.ethereum.wss_url.clone();
        let oracle_predeploy_address = conf.ethereum.oracle_predeploy_address;
        for (i, ct) in event.filter.cts.iter().enumerate() {
            let token = decrypt(
                &client_clone,
                &conf.clone(),
                event.filter.request_id,
                ct.ct_handle,
                ct.ct_type,
                i as i64,
                event.block_number,
            )
            .await
            .unwrap();
            tokens.push(token);
        }

        let wallet = WalletManager::default().wallet;
        let provider = Provider::<Ws>::connect(ethereum_wss_url).await.unwrap();
        let provider = SignerMiddleware::new(provider.clone(), wallet.with_chain_id(9000_u64));
        let provider = Arc::new(provider);
        let contract = GatewayContract::new(oracle_predeploy_address, Arc::clone(&provider));

        // Fake signatures for now
        let signatures = vec![Bytes::from(vec![0u8; 65])];

        tracing::info!("Fulfilling request: {:?}", event.filter.request_id);
        match contract
            .fulfill_request(event.filter.request_id, encode(&tokens).into(), signatures)
            .send()
            .await
        {
            Ok(pending_tx) => match pending_tx.await {
                Ok(receipt) => {
                    tracing::info!("Fulfilled request: {:?}", event.filter.request_id);
                    tracing::trace!("Transaction receipt: {:?}", receipt);
                }
                Err(e) => {
                    tracing::error!("Failed to await transaction receipt: {:?}", e);
                }
            },
            Err(e) => {
                tracing::error!("Failed to send fulfill_request transaction: {:?}", e);
            }
        }
    });
    tracing::debug!("🍻 handle_event_decryption exit");
    Ok(())
}

async fn decrypt(
    client: &Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
    config: &GatewayConfig,
    request_id: U256,
    ct_handle: U256,
    ct_type: u8,
    _ct_index: i64,
    block_number: u64,
) -> Result<Token, Box<dyn std::error::Error>> {
    let mut ct_handle_bytes = [0u8; 32];
    ct_handle.to_big_endian(&mut ct_handle_bytes);
    let ct_bytes =
        <EthereumConfig as Into<Box<dyn CiphertextProvider>>>::into(config.clone().ethereum)
            .get_ciphertext(client, ct_handle_bytes.to_vec(), block_number)
            .await?;
    let data_bytes = ct_bytes.to_vec();

    // Convert the Vec<u8> to a hex string
    let hex_data = hex::encode(&data_bytes);

    // Send the hex-encoded data to the Actix web service
    let response = reqwest::Client::new()
        .post(format!("{}/store", "http://localhost:8088"))
        .body(hex_data)
        .send()
        .await?;

    // Print the response
    let handle = response.text().await?;
    println!("Response: {}", handle);

    tracing::info!("📦 Stored ciphertext, handle: {}", handle);

    let handle_bytes = hex::decode(handle).unwrap();

    tracing::debug!("Got ct bytes of length: {}", ct_bytes.len());
    tracing::trace!("ct_bytes: 0x{}", hex::encode(&ct_bytes));
    tracing::info!("🚀 request_id: {}, ct_type: {}", request_id, ct_type,);
    Ok(blockchain_impl(config)
        .await
        .decrypt(handle_bytes, FheType::from(ct_type))
        .await?)
}
