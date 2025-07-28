use alloy::{
    primitives::{Address, B256, U64, U256, address, keccak256},
    signers::{Signer, local::PrivateKeySigner},
};
use fusion_plus_sdk::{
    chain_id::ChainId,
    cross_chain_order::{CrossChainOrderParams, PreparedOrder},
    hash_lock::HashLock,
    quote::QuoteRequest,
    sdk::FusionPlusSdk,
    utils::random::get_random_bytes32,
};

#[tokio::main]
pub async fn main() -> fusion_plus_sdk::Result<()> {
    let sdk = FusionPlusSdk::new(
        "https://api.1inch.dev/fusion-plus",
        "wIjShzXW71PD87qE4AyqZEvwBqyMmw4c",
    );

    let wallet = PrivateKeySigner::random();

    let quote_request = QuoteRequest::new(
        ChainId::Ethereum,
        ChainId::Arbitrum,
        address!("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"), // USDC mainnet
        address!("0xaf88d065e77c8cC2239327C5EDb3A432268e5831"), // USDC arbitrum
        U256::from(100e6),
        true,
        wallet.address(),
    );
    // println!("Quote Request: {quote_request:#?}");

    let quote_result = sdk.get_quote(&quote_request).await?;
    // println!("Quote Result: {quote_result:#?}");

    let secrets_count = quote_result.recommended_preset().secrets_count;
    let secrets: Vec<B256> = (0..secrets_count).map(|_| get_random_bytes32()).collect();
    let secret_hashes: Vec<B256> = secrets.iter().map(HashLock::hash_secret).collect();

    let hash_lock = if secrets_count == 1 {
        HashLock::for_single_fill(&secrets[0])
    } else {
        HashLock::for_multiple_fills(
            secrets
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let mut encoded = [0u8; 36];
                    encoded[0..4].copy_from_slice(U64::from(i).to_be_bytes::<4>().as_ref());
                    encoded[4..36].copy_from_slice(s.as_ref());

                    keccak256(encoded)
                })
                .collect(),
        )
        .unwrap()
    };

    let order = PreparedOrder::from_quote(
        &quote_request,
        &quote_result,
        CrossChainOrderParams {
            dst_address: Address::ZERO,
            hash_lock,
            secret_hashes: secret_hashes.clone(),
        },
    )
    .unwrap();

    println!("Order created: {order:#?}");

    let signature = wallet
        .sign_hash(&order.eip712_signing_hash())
        .await
        .unwrap();

    let result = sdk.submit_order(&order, &secret_hashes, &signature).await;

    Ok(())
}
