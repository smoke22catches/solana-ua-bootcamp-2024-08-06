import { Keypair } from "@solana/web3.js";
import "dotenv/config";

const privateKey = process.env["SECRET_KEY"];

if (!privateKey) {
  console.error("Secret key is not found!");
  process.exit(1);
}

const privateKeyAsArray = Uint8Array.from(JSON.parse(privateKey));
const keypair = Keypair.fromSecretKey(privateKeyAsArray);
console.log(`Public key: ${keypair.publicKey.toBase58()}`);
