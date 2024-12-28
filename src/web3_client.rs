use web3::transports::Http;
use web3::Web3;
use std::convert::TryFrom;
use std::sync::Arc;

use ethers::prelude::*;


pub async fn connect_to_ganache(rpc_url: &str) -> Web3<Http> {
    // let transport = Http::new(rpc_url).expect("Failed to create HTTP transport");
    // Web3::new(transport)
    Provider::<Http>::try_from(rpc_url).expect("Couldn't instantiate provider");
}

