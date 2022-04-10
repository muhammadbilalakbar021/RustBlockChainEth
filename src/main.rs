mod args;
use std::env;
use std::str::FromStr;

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

    // Connectinig to Ganache Server
    let transport: Http =
        web3::transports::Http::new(&env::var("GANACHE_SERVER").unwrap()).unwrap();
    let web3: Web3<Http> = web3::Web3::new(transport);

    println!("Getting Etherem Accounts.");
    #[allow(unused_variables)]
    let accounts = web3.eth().accounts().await?;
    // accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    // Get private Key from Arguments
    let private_key = &get_private_key(args.privatekey);

    // Get amount to transfer from Arguments
    // Covert the amount into WEI
    let amount_to_transfer = convert_amount_to_wei(args.amount.to_string());

    // Get Public Key from Private Key using clarity crate
    let bobs_key = PrivateKey::from_slice(private_key).unwrap();
    let public_key = &convert_web3_address(bobs_key.to_address().to_string());
    println!("Private Key of BOB{:?}", bobs_key);
    println!("Address of BOB{:?}", public_key);

    // let to_account_key = &convert_web3_address(args.to_account_key);

    // To Transfer into account is of type Str we have to convert the strinig into H160 vec
    // H160 = Fixed-size uninterpreted hash type with 20 bytes (160 bits) size.

    let to_account_key = web3::types::H160::from_str(&args.to_account_key).unwrap();

    // Check Balance of Account Before Signing the Tx
    #[warn(unused_variables)]
    let balance_before: web3::types::U256 = web3.eth().balance(to_account_key, None).await.unwrap();

    // Calculate the nonce using web3 and passing the derefrenced value of account address
    let nonce = web3
        .eth()
        .transaction_count(*public_key, None)
        .await
        .unwrap();

    // Creating Raw Transaction
    let tx: RawTransaction = RawTransaction {
        nonce: convert_u256(nonce),
        to: Some(convert_account(to_account_key)),
        value: ethereum_types::U256::from(i64::from(amount_to_transfer)),
        gas_price: ethereum_types::U256::from(1000000000),
        gas: ethereum_types::U256::from(21000),
        data: Vec::new(),
    };

    // Signing the Transaction using Private Key
    let signed_tx = tx.sign(private_key);

    // Hash of Signed Tx
    let tx_hash = web3
        .eth()
        .send_raw_transaction(Bytes::from(signed_tx))
        .await
        .unwrap();

    let balance_after = web3.eth().balance(to_account_key, None).await.unwrap();

    println!("TX Hash: {:?}", tx_hash);
    println!("Balance before: {}", balance_before);
    println!("Balance after: {}", balance_after);

    Ok(())
}

// Conversion of Web3 U256 to Rust U256
fn convert_u256(value: web3::types::U256) -> U256 {
    let web3::types::U256(ref arr) = value;
    let mut ret = [0; 4];
    ret[0] = arr[0];
    ret[1] = arr[1];
    U256(ret)
}

// Conversion of Web3 H160 to Rust H160
fn convert_account(value: web3::types::H160) -> H160 {
    let ret = H160::from(value.0);
    ret
}

// Conversion of String to Web3 H160
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

// Conversion of String to Web3 H256
fn get_private_key(key: String) -> H256 {
    let private_key = hex::decode(key).unwrap();
    return H256(to_array(private_key.as_slice()));
}

// Remove 0x from Address
fn parse0x_from_string(str: &str, pos: usize) -> &str {
    match str.char_indices().skip(pos).next() {
        Some((pos, _)) => &str[pos..],
        None => "",
    }
}

// Coversion of Amount which is in string to i64 Primitive Type
fn convert_amount_to_wei(amount: String) -> i64 {
    let amount_to_convert: f64 = amount.trim().parse().expect("Wanted a number");
    let value_to_transfer: f64 = amount_to_convert * 1000000000000000000.0;
    value_to_transfer as i64
}

// Conversion of WEI to f64
#[allow(dead_code)]
fn wei_to_eth(wei_val: U256) -> i64 {
    let res = wei_val.as_u32() as f64;
    (res / 1_000_000_000_000_000_000.0) as i64
}
