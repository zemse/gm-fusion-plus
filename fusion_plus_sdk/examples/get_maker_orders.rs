use fusion_plus_sdk::api::{Api, types::OrdersByMakerParams};

#[tokio::main]
pub async fn main() -> fusion_plus_sdk::Result<()> {
    dotenvy::from_path("../.env").unwrap();

    let api = Api::new(
        "https://api.1inch.dev/fusion-plus",
        std::env::var("ONEINCH_API_KEY").expect("ONEINCH_API_KEY not set in .env file"),
    );

    let result = api
        .get_orders_by_maker(
            "0x5bc44f18b91f55540d11d612c08e4faad619eb55"
                .parse()
                .unwrap(),
            OrdersByMakerParams::default().paginated(),
        )
        .await
        .unwrap();

    println!("Maker Orders: {result:#?}");

    Ok(())
}

// Maker Orders: PaginationOutput {
//     meta: PaginationMeta {
//         total_items: 13,
//         items_per_page: 100,
//         total_pages: 1,
//         current_page: 1,
//     },
//     items: [
//         OrderFillsByMakerOutput {
//             order_hash: "0x5e30e2d3db444542c04a3074c2eb68801c5d21a4bb624e7af60647fe0d08c60f",
//             validation: Valid,
//             status: Refunded,
//             maker_asset: "0xaf88d065e77c8cc2239327c5edb3a432268e5831",
//             taker_asset: "0x0b2c639c533813f4aa9d7837caf62653d097ff85",
//             maker_amount: "1000000",
//             min_taker_amount: "915736",
//             approximate_taking_amount: "947276",
//             cancel_tx: None,
//             fills: [
//                 Fill {
//                     status: Refunded,
//                     tx_hash: "0xa01d5b7443624af754a22463eed4dfb2512c2387aefd047256b8bedfea8abc79",
//                     filled_maker_amount: "1000000",
//                     filled_auction_taker_amount: "947276",
//                     escrow_events: [
//                         EscrowEventData {
//                             transaction_hash: "0xa01d5b7443624af754a22463eed4dfb2512c2387aefd047256b8bedfea8abc79",
//                             escrow: "0x736821e5cd0b8cd78e3af6131e48b9597f09e63a",
//                             side: Src,
//                             action: SrcEscrowCreated,
//                             block_timestamp: 1753951254000,
//                         },
//                         EscrowEventData {
//                             transaction_hash: "0xf9a23146210238ea2bcd40aebd007f0ac59781a00701dec770c89899f6629c34",
//                             escrow: "0x6ba09c73c65683b12242d0db4fdad258530e237c",
//                             side: Dst,
//                             action: DstEscrowCreated,
//                             block_timestamp: 1753951275000,
//                         },
//                         EscrowEventData {
//                             transaction_hash: "0x71309e2fea2b4b1ed1cb51ae6541b0f4781ac45fcff9b4cb242a13d69e204156",
//                             escrow: "0x6ba09c73c65683b12242d0db4fdad258530e237c",
//                             side: Dst,
//                             action: EscrowCancelled,
//                             block_timestamp: 1753951763000,
//                         },
//                         EscrowEventData {
//                             transaction_hash: "0x1ce0ab50dc662282fa63364a8e7695e1324457eb1560cc91af263883883826bd",
//                             escrow: "0x736821e5cd0b8cd78e3af6131e48b9597f09e63a",
//                             side: Src,
//                             action: EscrowCancelled,
//                             block_timestamp: 1753951833000,
//                         },
//                     ],
//                 },
//             ],
//             points: None,
//             auction_start_date: 1753951270,
//             auction_duration: 180,
//             initial_rate_bump: 696150,
//             is_native_currency: false,
//             src_chain_id: Arbitrum,
//             dst_chain_id: Optimism,
//             created_at: 1753951254130,
//             cancelable: false,
//         },
