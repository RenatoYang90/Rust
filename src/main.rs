use web3::types::{BlockId, BlockNumber, Transaction, TransactionReceipt, H160};
use web3::transports::Http;
use web3::Web3;

#[tokio::main]
async fn main() -> web3::Result<()> {
    // Connect to Ganache running on localhost:8545
    let transport = Http::new("http://127.0.0.1:7545")?;
    let web3 = Web3::new(transport);

    let block_number = web3.eth().block_number().await?;
    println!("Latest block number: {:?}", block_number);

    // Fetch the latest block
    if let Some(block) = web3.eth().block(BlockId::Number(BlockNumber::Latest)).await? {
        //println!("Block Details: {:?}", block);

        // Loop through transactions in the block
        
        for tx_hash in block.transactions {
            let tx = web3.eth().transaction(tx_hash.into()).await?;
            let receipt = web3.eth().transaction_receipt(tx_hash.into()).await?;

            println!("\nTransaction: {:?}", tx);
            println!("Receipt: {:?}", receipt);

            // Perform analysis (example: gas used)
            if let Some(r) = receipt {
                println!("Gas used: {:?}", r.gas_used);
            }
        }
        
    }

    Ok(())
}
