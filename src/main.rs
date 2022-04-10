mod args;
use args::Args;

extern crate clarity;
extern crate ethereum_tx_sign;
extern crate ethereum_types;
extern crate hex;
extern crate web3;

#[allow(unused_imports)]
use clarity::{Address, PrivateKey, Signature, Transaction};
use ethereum_tx_sign::RawTransaction;
#[allow(unused_imports)]
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
    let args = Args::new();

    let transport: Http = web3::transports::Http::new("http://localhost:7545").unwrap();
    let web3: Web3<Http> = web3::Web3::new(transport);

    println!("Getting Etherem Accounts.");
    let accounts = web3.eth().accounts().await?;
    // accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    let private_key = &get_private_key(args.privatekey);
    let amount_to_transfer = convert_amount_to_web3_eth_type(args.amount.to_string());

    println!("Info Of Account 0");
    println!("${:?}", accounts[0]);

    let bobs_key = PrivateKey::from_slice(private_key).unwrap();
    let public_key = &convert_web3_address(bobs_key.to_address().to_string());
    println!("Get Private Key.");
    println!("${:?}", bobs_key);
    println!("${:?}", public_key);

    let to_account_key = &convert_web3_address(args.to_account_key);
    #[warn(unused_variables)]
    let balance_before: web3::types::U256 =
        web3.eth().balance(*to_account_key, None).await.unwrap();
    let nonce = web3
        .eth()
        .transaction_count(*public_key, None)
        .await
        .unwrap();

    let tx: RawTransaction = RawTransaction {
        nonce: convert_u256(nonce),
        to: Some(convert_account(*to_account_key)),
        value: ethereum_types::U256::from(i64::from(amount_to_transfer)),
        gas_price: ethereum_types::U256::from(1000000000),
        gas: ethereum_types::U256::from(21000),
        data: Vec::new(),
    };

    let signed_tx = tx.sign(private_key);

    let tx_hash = web3
        .eth()
        .send_raw_transaction(Bytes::from(signed_tx))
        .await
        .unwrap();

    let balance_after = web3.eth().balance(*to_account_key, None).await.unwrap();

    println!("TX Hash: {:?}", tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after: {}", balance_after);

    Ok(())
}

fn convert_u256(value: web3::types::U256) -> U256 {
    let web3::types::U256(ref arr) = value;
    let mut ret = [0; 4];
    ret[0] = arr[0];
    ret[1] = arr[1];
    U256(ret)
}

fn convert_account(value: web3::types::H160) -> H160 {
    let ret = H160::from(value.0);
    ret
}

fn convert_web3_address(value: String) -> web3::types::H160 {
    let address_to_parse = parse0x_from_string(&value, 2);
    let address = hex::decode(address_to_parse).unwrap();
    return web3::types::H160(to_array_of8_bit_20(address.as_slice()));
}

fn to_array(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}

fn to_array_of8_bit_20(bytes: &[u8]) -> [u8; 20] {
    let mut array = [0; 20];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}

fn get_private_key(key: String) -> H256 {
    let private_key = hex::decode(key).unwrap();
    return H256(to_array(private_key.as_slice()));
}

fn parse0x_from_string(str: &str, pos: usize) -> &str {
    match str.char_indices().skip(pos).next() {
        Some((pos, _)) => &str[pos..],
        None => "",
    }
}

fn convert_amount_to_web3_eth_type(amount: String) -> i64 {
    let amount_to_convert: f64 = amount.trim().parse().expect("Wanted a number");
    let value_to_transfer: f64 = amount_to_convert * 1000000000000000000.0;
    value_to_transfer as i64
}
