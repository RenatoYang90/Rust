mod config;
//mod web3_client;
//mod trading;

use dotenv::dotenv;

use std::convert::TryFrom;
use std::sync::Arc;

use ethers::prelude::*;
//use eyre::Result;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let rpc_url = config::get_rpc_url();
    let private_key = config::get_private_key();

    let provider = Provider::<Http>::try_from(rpc_url).expect("Couldn't instantiate provider");

    // Optionally, increase the poll interval if needed.
    // This is how frequently the client checks for transaction confirmations.
    let provider = provider.interval(Duration::from_millis(1000));

    println!("Connected to Ganache!");

    let wallet: LocalWallet = private_key.parse().expect("Wallet address is not correct one.");
    let wallet = wallet.with_chain_id(1337u64);

    // Connect the wallet (signer) to the provider
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    //Define the recipient address
    let recipient: Address = "0xC6494D7b78eD8170592a2eDFe25f1b863917B3E0".parse().expect("Recipient address is not correct");

    let tx = TransactionRequest::pay(recipient, 1_000_000_000_000_000_000u128);

    println!("Sending transaction...");

    //Submit the transactio to Ganache
    let pending_tx = client.send_transaction(tx, None).await.expect("Transaction failed!!!");

    //Wait for transaction confirmation
    let receipt = pending_tx.confirmations(1).await.expect("Block creation failed!!!");

    if let Some(receipt) = receipt {
        println!("Transaction mined!");
        println!("Transaction hash: {:?}", receipt.transaction_hash);
        println!("Gas used: {:?}", receipt.gas_used);
    } else {
        println!("Transaction not confirmed yet...");
    }
}
