#![allow(deprecated)]

use env_logger::Env;
use ethers::prelude::*;
use std::{path::PathBuf, sync::Arc, str::FromStr};

use helios::{
    client::{Client, ClientBuilder, FileDB},
    config::networks::Network,
    types::{BlockTag, CallOpts},
};

// Generate the type-safe contract bindings with an ABI
abigen!(
    Renderer,
    r#"[
        function renderBroker(uint256) external view returns (string memory)
        function renderBroker(uint256, uint256) external view returns (string memory)
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);


abigen!(
    ERC20Token, "./examples/erc20.json",
    event_derives(serde::Deserialize, serde::Serialize)

);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // Load the rpc url using the `MAINNET_EXECUTION_RPC` environment variable
    let eth_rpc_url = std::env::var("MAINNET_EXECUTION_RPC")?;
    let consensus_rpc = "http://testing.prater.beacon-api.nimbus.team";
    log::info!("Consensus RPC URL: {}", consensus_rpc);

    // Construct the client
    let data_dir = PathBuf::from("/tmp/helios");


    use ethers::prelude::Abigen;
    Abigen::new("ERC20Token", "examples/erc20.json")?.generate()?.write_to_file("token.rs")?;

    // Start the client

    //let contract = ERC20Token::new(address, client);

    // Call the erroneous account method
    // The expected asset is: https://0x8bb9a8baeec177ae55ac410c429cbbbbb9198cac.w3eth.io/renderBroker/5
    // Retrieved by calling `renderBroker(5)` on the contract: https://etherscan.io/address/0x8bb9a8baeec177ae55ac410c429cbbbbb9198cac#code
    let account = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
    let address = account.parse::<Address>()?;

    let method = "renderBroker(uint256)";
    let method2 = "renderBroker(uint256, uint256)";
    //let argument = U256::from(5);
    let argument = "0x9E32Cca3C8cF7434d99629448360b8AC3Db50118".parse::<Address>()?;
    let block = BlockTag::Latest;
    let provider = Provider::<Http>::try_from(eth_rpc_url.clone())?;
    let render = ERC20Token::new(address, Arc::new(provider.clone()));
    log::debug!("Context: call @ {account}::{method} <{argument}>");
    //
    // Call using abigen
    //let result = render.balance_of(argument).call().await?;
   // log::info!(
    //    "[ABIGEN] {account}::{method} -> Response: {:?}",
     //   result
    //);


    let mut client: Client<FileDB> = ClientBuilder::new()
        .network(Network::GOERLI)
        .data_dir(data_dir)
        .consensus_rpc(consensus_rpc)
        .execution_rpc(&eth_rpc_url)
        .checkpoint("0x7e8d04a93be4eb2fdc7492c544ded2eb1cb36185179d1ab4ea598e1ad7d7793c")
        //.load_external_fallback()
        .build()?;
    log::info!(
        "[\"{}\"] Client built with external checkpoint fallbacks",
        Network::MAINNET
    );    client.start().await?;

    let b = client.get_balance(&argument, BlockTag::Latest).await.unwrap();
    println!("B {}", b);

    /*

    // Call on helios client
    let encoded_call = render.balance_of(argument).calldata().unwrap();
    let call_opts = CallOpts {
        //from: Some("0xBE0eB53F46cd790Cd13851d5EFf43D12404d33E8".parse::<Address>()?),
        //to: Some(address),
        from: None,
        to: Some(address),
        gas: Some(U256::from(U64::MAX.as_u64())),
        gas_price: None,
        value: None,
        data: Some(encoded_call.to_vec()),
    };
    log::debug!("Calling helios client on block: {block:?}");
    let result: [u8; 32] = client.call(&call_opts, block).await?.try_into().unwrap();
    log::info!("[HELIOS] {account}::{method}  ->{:?}", U256::from(result));

    let head_block_num = client.get_block_number().await?;
    //let addr = Address::from_str("0x00000000219ab540356cBB839Cbe05303d7705Fa")?;
    let block = BlockTag::Latest;
    //let balance = client.get_storage_at(&address, H256::from_str("0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3").unwrap(), block).await?;

        //let usdc_impl = "0xa2327a938febf5fec13bacfb16ae10ecbc4cbdcf".parse::<Address>()?;
        let usdc_impl = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse::<Address>()?;
    let balance = client.get_storage_at(&usdc_impl, H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(), block).await?;
                log::info!( "balance of deposit contract: {:#x}", balance);

        let index = H256::from_low_u64_be(9);
        let key =   H256::from_str("000000000000000000000000a9d1e08c7793af67e9d92fe308d5697fb81d3e43").unwrap();

        let mut k = Vec::new();
        k.extend_from_slice(key.as_bytes());
        k.extend_from_slice(index.as_bytes());
        let h = H256(ethers::utils::keccak256(k));

        

        let balance = client.get_storage_at(&usdc_impl, h, block).await?;
        println!("WWWWWWW {}", balance);


    //
    //
    //
    //
    //
    for i in 0..100 {
        let index = H256::from_low_u64_be(i);
        //let key =   H256::from_str("000000000000000000000000a9d1e08c7793af67e9d92fe308d5697fb81d3e43").unwrap();

        let mut k = Vec::new();
        //k.extend_from_slice(key.as_bytes());
        k.extend_from_slice(index.as_bytes());
        //let h = H256(ethers::utils::keccak256(k));
        let h = index;

        

        let balance = client.get_storage_at(&usdc_impl, h, block).await?;

                log::info!( "balance of deposit contract: {} = {:#x}", i, balance);
    }

*/
    Ok(())
}
