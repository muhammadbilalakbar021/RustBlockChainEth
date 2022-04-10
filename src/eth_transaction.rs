extern crate ethereum_tx_sign;
extern crate ethereum_types;
extern crate hex;
extern crate web3;

use ethereum_tx_sign::RawTransaction;
use ethereum_types::{H160, H256, U256, U512};
#[allow(unused_imports)]
use web3::futures::Future;
use web3::transports::Http;
use web3::types::Bytes;
#[allow(unused_imports)]
use web3::types::TransactionRequest;
use web3::Web3;

// Unwrap() Give me the result of the computation, and if there was an error, panic and stop the program
#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);

    println!("Getting Etherem Accounts.");
    let accounts = web3.eth().accounts().await?;
    // accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    println!("Info Of Account 0");
    println!("${:?}", accounts[0]);
    println!("Calling balance.");

    #[warn(unused_variables)]
    let balance_before = web3.eth().balance(accounts[1], None).await.unwrap();

    let tx = TransactionRequest {
        from: accounts[0],
        to: Some(accounts[1]),
        gas: None,
        gas_price: None,
        value: Some(U256::from(10000)),
        data: None,
        nonce: None,
        condition: None,
        transaction_type: None,
        access_list: None,
    };

    let tx_hash = web3.eth().send_transaction(tx).await.unwrap();
    let balance_after = web3.eth().balance(accounts[1], None).await.unwrap();

    println!("TX Hash: {:?}", tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after:  {}", balance_after);

    Ok(())
}
