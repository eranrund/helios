// use ethers::abi;
// use ethers::abi::Token;
// use ethers::signers::Signer;
// use ethers::signers::Wallet;
// use ethers::utils::keccak256;
use std::error::Error;

use ethers_core::{
    abi::{self, Token},
    k256::ecdsa::SigningKey,
    k256::{ecdsa::signature::hazmat::PrehashSigner, elliptic_curve::FieldBytes, Secp256k1},
    types::{Signature, H256, U256},
    utils::{hash_message, keccak256},
};

pub fn sign_hash(signing_key: &SigningKey, hash: H256) -> Result<Signature, ()> {
    let (recoverable_sig, recovery_id) = signing_key.sign_prehash(hash.as_ref()).unwrap();

    let v = u8::from(recovery_id) as u64 + 27;

    let r_bytes: FieldBytes<Secp256k1> = recoverable_sig.r().into();
    let s_bytes: FieldBytes<Secp256k1> = recoverable_sig.s().into();
    let r = U256::from_big_endian(r_bytes.as_slice());
    let s = U256::from_big_endian(s_bytes.as_slice());

    Ok(Signature { r, s, v })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let key_bytes = hex::decode(".....").unwrap();

    let encoded = keccak256(abi::encode_packed(&[Token::String("hello".into())])?);

    //let wallet = Wallet::from_bytes(&key_bytes).unwrap();
    //let signature = wallet.sign_message(encoded).await?;

    let key_bytes2: &[u8] = &key_bytes[..];
    let signer = SigningKey::from_bytes(key_bytes2.into()).unwrap();

    let message_hash = hash_message(encoded);
    let signature = sign_hash(&signer, message_hash).unwrap();

    println!("Signature: {}", signature);

    Ok(())
}
