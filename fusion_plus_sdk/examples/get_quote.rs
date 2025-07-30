use alloy::primitives::{Address, U256};
use fusion_plus_sdk::{addresses::usdc, api::Api, chain_id::ChainId, quote::QuoteRequest};

#[tokio::main]
async fn main() -> fusion_plus_sdk::Result<()> {
    dotenvy::from_path("../.env").unwrap();

    let api = Api::new(
        "https://api.1inch.dev/fusion-plus",
        std::env::var("ONEINCH_API_KEY").expect("ONEINCH_API_KEY not set in .env file"),
    );

    let result = api
        .get_quote(&QuoteRequest::new(
            ChainId::Optimism,
            ChainId::Arbitrum,
            usdc(ChainId::Optimism),
            usdc(ChainId::Arbitrum),
            U256::from(1e6),
            true,
            Address::ZERO,
        ))
        .await?;

    println!("Quote Result: {result:#?}");

    Ok(())
}

// Quote Result: QuoteResult {
//     quote_id: String("066b4b2c-6719-4173-adf0-2844a396f98b"),
//     src_token_amount: "100000000",
//     dst_token_amount: "98967102",
//     presets: QuotePresets {
//         fast: Preset {
//             auction_duration: 180.0,
//             start_auction_in: 24.0,
//             initial_rate_bump: 163812.0,
//             auction_start_amount: "99576228",
//             start_amount: "98967101",
//             auction_end_amount: "97971339",
//             exclusive_resolver: Null,
//             cost_in_dst_token: "609133",
//             points: [
//                 AuctionPoint {
//                     delay: 120.0,
//                     coefficient: 124562.0,
//                 },
//                 AuctionPoint {
//                     delay: 60.0,
//                     coefficient: 62174.0,
//                 },
//             ],
//             allow_partial_fills: false,
//             allow_multiple_fills: false,
//             gas_cost: GasCostConfig {
//                 gas_bump_estimate: 62174.0,
//                 gas_price_estimate: "588",
//             },
//             secrets_count: 1,
//         },
//         medium: Preset {
//             auction_duration: 360.0,
//             start_auction_in: 24.0,
//             initial_rate_bump: 163812.0,
//             auction_start_amount: "99576228",
//             start_amount: "98967101",
//             auction_end_amount: "97971339",
//             exclusive_resolver: Null,
//             cost_in_dst_token: "609133",
//             points: [
//                 AuctionPoint {
//                     delay: 360.0,
//                     coefficient: 62174.0,
//                 },
//             ],
//             allow_partial_fills: false,
//             allow_multiple_fills: false,
//             gas_cost: GasCostConfig {
//                 gas_bump_estimate: 62174.0,
//                 gas_price_estimate: "588",
//             },
//             secrets_count: 1,
//         },
//         slow: Preset {
//             auction_duration: 600.0,
//             start_auction_in: 24.0,
//             initial_rate_bump: 163812.0,
//             auction_start_amount: "99576228",
//             start_amount: "98967101",
//             auction_end_amount: "97971339",
//             exclusive_resolver: Null,
//             cost_in_dst_token: "609133",
//             points: [
//                 AuctionPoint {
//                     delay: 600.0,
//                     coefficient: 62174.0,
//                 },
//             ],
//             allow_partial_fills: false,
//             allow_multiple_fills: false,
//             gas_cost: GasCostConfig {
//                 gas_bump_estimate: 62174.0,
//                 gas_price_estimate: "588",
//             },
//             secrets_count: 1,
//         },
//         custom: None,
//     },
//     src_escrow_factory: "0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a",
//     dst_escrow_factory: "0xa7bcb4eac8964306f9e3764f67db6a7af6ddf99a",
//     whitelist: [
//         "0xf24fcc6cb06a1e061bc1cb4fa6eb00f6ea887a4a",
//         "0x33b41fe18d3a39046ad672f8a0c8c415454f629c",
//         "0x77774bebe35057beb2545ba74b09ae44e823cf77",
//     ],
//     time_locks: TimeLocks {
//         src_withdrawal: 36.0,
//         src_public_withdrawal: 372.0,
//         src_cancellation: 528.0,
//         src_public_cancellation: 648.0,
//         dst_withdrawal: 60.0,
//         dst_public_withdrawal: 336.0,
//         dst_cancellation: 456.0,
//     },
//     src_safety_deposit: "123635832390000",
//     dst_safety_deposit: "2161036080000",
//     recommended_preset: Fast,
//     prices: PairCurrency {
//         usd: TokenPair {
//             src_token: "0.9965449135610982",
//             dst_token: "1.0000846596826491",
//         },
//     },
//     volume: PairCurrency {
//         usd: TokenPair {
//             src_token: "99.65",
//             dst_token: "99.58",
//         },
//     },
// }
