use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use ethers::{
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    contract::Contract,
    types::Address,
};

abigen!(
    IUniswapV2Pair,
    "[function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)]"
);

pub async fn monitor(contract_address: Address, provider: Arc<Provider<Http>>) {
    // Initialize a new instance of the Weth/Dai Uniswap V2 pair contract
    let contract = IUniswapV2Pair::new(contract_address, provider);

    loop {
        if let Ok(result) = contract.get_reserves().call().await {
            println!("Result: {:?}", result);
            // Handle the result as needed
        }
        sleep(Duration::from_secs(24*3600)).await;
    }
}