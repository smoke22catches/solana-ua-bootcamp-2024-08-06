use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use dotenv::dotenv;
use std::{env, str::FromStr};

fn main() {
    dotenv().ok();

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let account_public_key_str = env::var("ACCOUNT_PUBLIC_KEY").expect("Account public key should be set up");
    let account_public_key = Pubkey::from_str(&account_public_key_str).expect("Invalid public key");

    match client.get_balance(&account_public_key) {
        Ok(balance) => {
            println!("Balance: {}", balance);
        }
        Err(err) => {
            eprintln!("Error recieving balance: {}", err);
        }
    }
}
