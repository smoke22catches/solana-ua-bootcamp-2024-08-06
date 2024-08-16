use solana_sdk::{signature::Keypair, signer::Signer};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let account_private_key_str = env::var("ACCOUNT_PRIVATE_KEY").expect("Account private key should be set up");
    let account_private_key = account_private_key_str.split(", ").map(|bit| bit.parse::<u8>().unwrap() ).collect::<Vec<u8>>();
    
    let keypair = Keypair::from_bytes(&account_private_key).unwrap();


    println!("Public key is: {}", keypair.try_pubkey().ok().unwrap().to_string());
}