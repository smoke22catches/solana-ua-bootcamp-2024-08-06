use solana_sdk::signer::{keypair::Keypair, Signer};

fn main() {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let privkey = keypair.secret().as_bytes();
    println!("Public key is: {}", pubkey);
    println!("Private key is: {:?}", privkey);
    println!("Private key is: {:?}", keypair.to_bytes());
}