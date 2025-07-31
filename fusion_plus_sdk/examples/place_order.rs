use std::str::FromStr;

use alloy::{
    primitives::{Address, B256, U64, U256, keccak256},
    signers::{Signer, local::PrivateKeySigner},
};
use fusion_plus_sdk::{
    addresses::{get_limit_order_contract_address, usdc},
    api::Api,
    chain_id::ChainId,
    cross_chain_order::{CrossChainOrderParams, Fee, PreparedOrder},
    hash_lock::HashLock,
    quote::QuoteRequest,
    relayer_request::RelayerRequest,
    utils::{
        alloy::{ERC20, create_provider},
        random::get_random_bytes32,
    },
};

#[tokio::main]
pub async fn main() -> fusion_plus_sdk::Result<()> {
    dotenvy::dotenv().ok();

    let api = Api::new(
        "https://api.1inch.dev/fusion-plus",
        std::env::var("ONEINCH_API_KEY").expect("ONEINCH_API_KEY not set in .env file"),
    );

    let wallet = PrivateKeySigner::from_str(
        &std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env file"),
    )
    .unwrap();

    let quote_request = QuoteRequest::new(
        ChainId::Arbitrum,
        ChainId::Optimism,
        usdc(ChainId::Arbitrum),
        usdc(ChainId::Optimism),
        U256::from(1e6),
        true,
        wallet.address(),
    );
    println!("Quote Request: {quote_request:#?}");

    let quote_result = api.get_quote(&quote_request).await?;
    println!("Quote Result: {quote_result:#?}");

    let arb = create_provider(ChainId::Arbitrum, wallet.clone());

    let usdc_arb = ERC20::new(usdc(ChainId::Arbitrum), &arb);
    let spender = get_limit_order_contract_address(quote_request.src_chain_id);
    let allowance = usdc_arb
        .allowance(wallet.address(), spender)
        .call()
        .await
        .unwrap();
    if allowance < quote_request.src_amount {
        println!("Approving tokens for escrow factory...");
        usdc_arb
            .approve(spender, U256::MAX)
            .send()
            .await
            .unwrap()
            .watch()
            .await
            .unwrap();
        println!("Token approved.");
    } else {
        println!("Token already approved for escrow factory.");
    }

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
            dst_address: wallet.address(),
            hash_lock,
            secret_hashes: secret_hashes.clone(),
            fee: Some(Fee {
                taking_fee_bps: 100,
                taking_fee_receiver: Address::ZERO,
            }),
        },
    )
    .unwrap();

    println!("Order created: {order:#?}");

    let order_hash = order.eip712_signing_hash();
    let signature = wallet.sign_hash(&order_hash).await.unwrap();

    let rr = RelayerRequest::from_prepared_order(
        &order,
        signature,
        quote_result.quote_id.clone().unwrap(),
        if secret_hashes.len() == 1 {
            None
        } else {
            Some(secret_hashes)
        },
    );

    println!("Relayer Request: {rr:#?}");
    api.submit_order(rr).await?;
    println!("submit_order success");

    loop {
        let mut done = false;
        let read = api.get_ready_to_accept_secret_fills(&order_hash).await?;
        println!("Number of fills: {}", read.fills.len());
        for fill in read.fills {
            println!("Fill {fill:#?}");
            api.submit_secret(&order_hash, &secrets[fill.idx as usize])
                .await?;
            println!("secret submitted");
            done = true;
        }
        if done {
            break;
        }
    }

    Ok(())
}
