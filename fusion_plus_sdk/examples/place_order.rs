use std::str::FromStr;

use alloy::{
    primitives::{B256, U64, U256, keccak256},
    signers::{Signer, local::PrivateKeySigner},
};
use fusion_plus_sdk::{
    addresses::{get_limit_order_contract_address, usdc},
    api::Api,
    chain_id::ChainId,
    cross_chain_order::{CrossChainOrderParams, Fee, PreparedOrder},
    hash_lock::HashLock,
    multichain_address::MultichainAddress,
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

    let usdc_arb = ERC20::new(usdc(ChainId::Arbitrum).as_raw(), &arb);
    let spender = get_limit_order_contract_address(quote_request.src_chain_id);
    let allowance = usdc_arb
        .allowance(wallet.address(), spender.as_raw())
        .call()
        .await
        .unwrap();
    if allowance < quote_request.src_amount {
        println!("Approving tokens for escrow factory...");
        usdc_arb
            .approve(spender.as_raw(), U256::MAX)
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
            dst_address: MultichainAddress::from_raw(wallet.address()),
            hash_lock,
            secret_hashes: secret_hashes.clone(),
            fee: Some(Fee {
                taking_fee_bps: 100,
                taking_fee_receiver: MultichainAddress::ZERO,
            }),
            preset: None,
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

// Quote Request: QuoteRequest {
//     src_chain_id: Arbitrum,
//     dst_chain_id: Optimism,
//     src_token_address: 0xaf88d065e77c8cc2239327c5edb3a432268e5831,
//     dst_token_address: 0x0b2c639c533813f4aa9d7837caf62653d097ff85,
//     src_amount: 1000000,
//     enable_estimate: true,
//     maker_address: 0x5bc44f18b91f55540d11d612c08e4faad619eb55,
//     permit: None,
//     fee: None,
//     source: Some(
//         "gm/rust-sdk",
//     ),
//     is_permit2: None,
// }
// Quote Result: QuoteResult {
//     quote_id: Some(
//         "5cc8c723-c7d9-4227-af46-3b7b2139131c",
//     ),
//     src_token_amount: 1000000,
//     dst_token_amount: 966198,
//     presets: QuotePresets {
//         fast: Preset {
//             auction_duration: 180,
//             start_auction_in: 17,
//             initial_rate_bump: 263576,
//             auction_start_amount: 978904,
//             start_amount: 966198,
//             auction_end_amount: 953765,
//             exclusive_resolver: None,
//             cost_in_dst_token: 12706,
//             points: [
//                 AuctionPoint {
//                     delay: 120,
//                     coefficient: 191131,
//                 },
//                 AuctionPoint {
//                     delay: 60,
//                     coefficient: 133219,
//                 },
//             ],
//             allow_partial_fills: false,
//             allow_multiple_fills: false,
//             gas_cost: GasCostConfig {
//                 gas_bump_estimate: 133219,
//                 gas_price_estimate: 10,
//             },
//             secrets_count: 1,
//         },
//         medium: Preset {
//             auction_duration: 360,
//             start_auction_in: 17,
//             initial_rate_bump: 263576,
//             auction_start_amount: 978904,
//             start_amount: 966198,
//             auction_end_amount: 953765,
//             exclusive_resolver: None,
//             cost_in_dst_token: 12706,
//             points: [
//                 AuctionPoint {
//                     delay: 360,
//                     coefficient: 133219,
//                 },
//             ],
//             allow_partial_fills: false,
//             allow_multiple_fills: false,
//             gas_cost: GasCostConfig {
//                 gas_bump_estimate: 133219,
//                 gas_price_estimate: 10,
//             },
//             secrets_count: 1,
//         },
//         slow: Preset {
//             auction_duration: 600,
//             start_auction_in: 17,
//             initial_rate_bump: 263576,
//             auction_start_amount: 978904,
//             start_amount: 966198,
//             auction_end_amount: 953765,
//             exclusive_resolver: None,
//             cost_in_dst_token: 12706,
//             points: [
//                 AuctionPoint {
//                     delay: 600,
//                     coefficient: 133219,
//                 },
//             ],
//             allow_partial_fills: false,
//             allow_multiple_fills: false,
//             gas_cost: GasCostConfig {
//                 gas_bump_estimate: 133219,
//                 gas_price_estimate: 10,
//             },
//             secrets_count: 1,
//         },
//         custom: None,
//     },
//     src_escrow_factory: 0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a,
//     dst_escrow_factory: 0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a,
//     whitelist: [
//         0xd06298edb2e694b1266bf0fc4b2dbe711e518acb,
//         0x33b41fe18d3a39046ad672f8a0c8c415454f629c,
//         0x77774bebe35057beb2545ba74b09ae44e823cf77,
//     ],
//     time_locks: TimeLocks {
//         src_withdrawal: 60,
//         src_public_withdrawal: 420,
//         src_cancellation: 576,
//         src_public_cancellation: 696,
//         dst_withdrawal: 60,
//         dst_public_withdrawal: 360,
//         dst_cancellation: 480,
//         deployed_at: 0,
//     },
//     src_safety_deposit: 2226588000000,
//     dst_safety_deposit: 472576650000,
//     recommended_preset: Fast,
//     prices: PairCurrency {
//         usd: TokenPair {
//             src_token: "1.000112198303285",
//             dst_token: "1.0007247179813183",
//         },
//     },
//     volume: PairCurrency {
//         usd: TokenPair {
//             src_token: "1",
//             dst_token: "0.98",
//         },
//     },
// }
// Token already approved for escrow factory.
// Order created: PreparedOrder {
//     src_chain_id: Arbitrum,
//     order: CrossChainOrder {
//         inner: FusionOrder {
//             settlement_extension_contract: 0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a,
//             extension: EscrowExtension {
//                 fusion_extension: FusionExtension {
//                     settlement_extension_contract: 0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a,
//                     auction_details: AuctionDetails {
//                         start_time: 1754156507,
//                         duration: 180,
//                         initial_rate_bump: 263576,
//                         points: [
//                             AuctionPoint {
//                                 delay: 120,
//                                 coefficient: 191131,
//                             },
//                             AuctionPoint {
//                                 delay: 60,
//                                 coefficient: 133219,
//                             },
//                         ],
//                         gas_cost: GasCostConfig {
//                             gas_bump_estimate: 133219,
//                             gas_price_estimate: 10,
//                         },
//                     },
//                     post_interaction_data: SettlementPostInteractionData {
//                         whitelist: [
//                             WhitelistItem {
//                                 address_half: 0xf0fc4b2dbe711e518acb,
//                                 delay: 0,
//                             },
//                             WhitelistItem {
//                                 address_half: 0x72f8a0c8c415454f629c,
//                                 delay: 0,
//                             },
//                             WhitelistItem {
//                                 address_half: 0x5ba74b09ae44e823cf77,
//                                 delay: 0,
//                             },
//                         ],
//                         integrator_fee: Some(
//                             IntegratorFee {
//                                 receiver: 0x0000000000000000000000000000000000000000,
//                                 ratio: 0,
//                             },
//                         ),
//                         bank_fee: Some(
//                             0,
//                         ),
//                         resolving_start_time: 1754156490,
//                         custom_receiver: Some(
//                             0x5bc44f18b91f55540d11d612c08e4faad619eb55,
//                         ),
//                     },
//                     maker_permit: None,
//                 },
//                 hash_lock_info: HashLock {
//                     hash: 0x37c320f7b010ff48cc9fa5f651640cfce1adc48a301886abd688cc8117d74d79,
//                 },
//                 dst_chain_id: Optimism,
//                 dst_token: 0x0b2c639c533813f4aa9d7837caf62653d097ff85,
//                 src_safety_deposit: 2226588000000,
//                 dst_safety_deposit: 472576650000,
//                 time_locks: TimeLocks {
//                     src_withdrawal: 60,
//                     src_public_withdrawal: 420,
//                     src_cancellation: 576,
//                     src_public_cancellation: 696,
//                     dst_withdrawal: 60,
//                     dst_public_withdrawal: 360,
//                     dst_cancellation: 480,
//                     deployed_at: 0,
//                 },
//             },
//             inner: LimitOrder {
//                 salt: 59760570324334168870571149440013523821369857597971969449377342064151251341468,
//                 maker: 0x5bc44f18b91f55540d11d612c08e4faad619eb55,
//                 receiver: 0x0000000000000000000000000000000000000000,
//                 maker_asset: 0xaf88d065e77c8cc2239327c5edb3a432268e5831,
//                 taker_asset: 0xda0000d4000015a526378bb6fafc650cea5966f8,
//                 making_amount: 1000000,
//                 taking_amount: 953765,
//                 maker_traits: MakerTraits {
//                     value: 62419173104490761595518734107164453327180858579398760566479924277126082068480,
//                 },
//             },
//         },
//     },
//     hash: 0x8268f083d495f9c437cd9180ade511f6b28af2a7710b1dc9d2c1a88a15fd5020,
//     quote_id: "5cc8c723-c7d9-4227-af46-3b7b2139131c",
// }
// Relayer Request: RelayerRequest {
//     src_chain_id: Arbitrum,
//     order: Order {
//         salt: 59760570324334168870571149440013523821369857597971969449377342064151251341468,
//         maker: 0x5bc44f18b91f55540d11d612c08e4faad619eb55,
//         receiver: 0x0000000000000000000000000000000000000000,
//         makerAsset: 0xaf88d065e77c8cc2239327c5edb3a432268e5831,
//         takerAsset: 0xda0000d4000015a526378bb6fafc650cea5966f8,
//         makingAmount: 1000000,
//         takingAmount: 953765,
//         makerTraits: 62419173104490761595518734107164453327180858579398760566479924277126082068480,
//     },
//     signature: 0x701bb96d2147a7d87e88f7f086ad80f818ba04e4c4a86e727d6596048b1ae98e15cc619d4669eba307d0caf352b04dfb44d1d496a2c382155bccb8feb78452191b,
//     quote_id: "5cc8c723-c7d9-4227-af46-3b7b2139131c",
//     extension: 0x0000013b0000005e0000005e0000005e0000005e0000002f0000000000000000a7bcb4eac8964306f9e3764f67db6a7af6ddf99a0208630000000a688e4ddb0000b404059802ea9b0078020863003ca7bcb4eac8964306f9e3764f67db6a7af6ddf99a0208630000000a688e4ddb0000b404059802ea9b0078020863003ca7bcb4eac8964306f9e3764f67db6a7af6ddf99a688e4dcaf0fc4b2dbe711e518acb000072f8a0c8c415454f629c00005ba74b09ae44e823cf7700001837c320f7b010ff48cc9fa5f651640cfce1adc48a301886abd688cc8117d74d79000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000b2c639c533813f4aa9d7837caf62653d097ff850000000000000000000002066afc9f0000000000000000000000006e07c36b1000000000000001e0000001680000003c000002b800000240000001a40000003c,
//     secret_hashes: None,
// }
// submit_order success
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 0
// Number of fills: 1
// Fill ReadyToAcceptSecretFill {
//     idx: 0,
//     src_escrow_deploy_tx_hash: "0xa9d47faacceb9566349ccdc4b7881b77b718eb3d32cfa6744a7613c3972ee070",
//     dst_escrow_deploy_tx_hash: "0x32127ba2f17bab6ae80e311c6f543dcc1d435a0fd82e8c502575a603e6b3773a",
// }
// secret submitted
