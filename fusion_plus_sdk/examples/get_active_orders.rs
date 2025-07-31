use fusion_plus_sdk::{
    api::{Api, types::ActiveOrdersRequestParams},
    chain_id::ChainId,
};

#[tokio::main]
pub async fn main() -> fusion_plus_sdk::Result<()> {
    dotenvy::from_path("../.env").unwrap();

    let api = Api::new(
        "https://api.1inch.dev/fusion-plus",
        std::env::var("ONEINCH_API_KEY").expect("ONEINCH_API_KEY not set in .env file"),
    );

    let result = api
        .get_active_orders(
            ActiveOrdersRequestParams {
                src_chain_id: Some(ChainId::Arbitrum),
                dst_chain_id: Some(ChainId::Optimism),
            }
            .paginated(),
        )
        .await?;

    println!("Active Orders: {result:#?}");

    Ok(())
}

// Active Orders: PaginationOutput {
//     meta: PaginationMeta {
//         total_items: 1,
//         items_per_page: 100,
//         total_pages: 1,
//         current_page: 1,
//     },
//     items: [
//         ActiveOrder {
//             quote_id: "b8df822c-6dc2-4a75-ac2a-fa9269174cab",
//             order_hash: 0x5e30e2d3db444542c04a3074c2eb68801c5d21a4bb624e7af60647fe0d08c60f,
//             signature: 0x435c220f9031887cca799f18c39f1e63ee9dedb2d9daa869c784443c5d9386c747eec1a0cc9bf8f623a03a35555665d8398f49537cc2a48dd03d5cee55a7d7fb1b,
//             deadline: "+011974-06-29T03:30:12.000Z",
//             auction_start_date: "2025-07-31T08:41:10.000Z",
//             auction_end_date: "2025-07-31T08:44:10.000Z",
//             remaining_maker_amount: 1000000,
//             maker_balance: 7000000,
//             maker_allowance: 115792089237316195423570985008687907853269984665640564039457584007913116639935,
//             order: Order {
//                 salt: 59760570321745959693867245748379178318940997668073785827538330995644313864705,
//                 maker: 0x5bc44f18b91f55540d11d612c08e4faad619eb55,
//                 receiver: 0x0000000000000000000000000000000000000000,
//                 makerAsset: 0xaf88d065e77c8cc2239327c5edb3a432268e5831,
//                 takerAsset: 0xda0000d4000015a526378bb6fafc650cea5966f8,
//                 makingAmount: 1000000,
//                 takingAmount: 915736,
//                 makerTraits: 62419173104490761595518734107435110408028858411109332347501650304779241390080,
//             },
//             extension: 0x0000013b0000005e0000005e0000005e0000005e0000002f0000000000000000a7bcb4eac8964306f9e3764f67db6a7af6ddf99a055df00000000a688b2c260000b40a9f5607b40f0078055df0003ca7bcb4eac8964306f9e3764f67db6a7af6ddf99a055df00000000a688b2c260000b40a9f5607b40f0078055df0003ca7bcb4eac8964306f9e3764f67db6a7af6ddf99a688b2c15f0fc4b2dbe711e518acb000072f8a0c8c415454f629c00005ba74b09ae44e823cf770000187ce799a15840fbd2a45481a80d4755b0fb99107f03199655de7732abe605098a000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000b2c639c533813f4aa9d7837caf62653d097ff850000000000000000000004639818420000000000000000000000006e0c55012000000000000001e0000001680000003c000002b800000240000001a40000003c,
//             src_chain_id: Arbitrum,
//             dst_chain_id: Optimism,
//             is_maker_contract: false,
//             secret_hashes: None,
//             fills: [],
//         },
//     ],
// }
