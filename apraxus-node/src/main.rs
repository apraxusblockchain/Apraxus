use ed25519_dalek::{
    Signature, Signer, SigningKey, Verifier, VerifyingKey,
};
use getrandom::{rand_core::UnwrapErr, SysRng};
use sha2::{Digest, Sha256};

struct Wallet {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    address: String,
}

impl Wallet {
    fn new() -> Self {
        let mut rng = UnwrapErr(SysRng);
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let mut hasher = Sha256::new();
        hasher.update(verifying_key.as_bytes());

        let address = hasher
            .finalize()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        Wallet {
            signing_key,
            verifying_key,
            address,
        }
    }

    fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    fn verify(
        verifying_key: &VerifyingKey,
        message: &[u8],
        signature: &Signature,
    ) -> bool {
        verifying_key.verify(message, signature).is_ok()
    }
}

fn main() {
    println!("=================================");
    println!("       APRAXUS WALLET TEST");
    println!("=================================");

    let wallet = Wallet::new();

    println!("Wallet created!");
    println!();
    println!("APXV Address:");
    println!("{}", wallet.address);

    let message = b"Alice -> Bob : 25 APXV";

    println!();
    println!("Message:");
    println!("{}", String::from_utf8_lossy(message));

    let signature = wallet.sign(message);

    println!();
    println!("Digital signature created.");

    let valid = Wallet::verify(
        &wallet.verifying_key,
        message,
        &signature,
    );

    println!();
    println!("Signature valid: {}", valid);

    let fake_message = b"Alice -> Bob : 1000 APXV";

    let fake_valid = Wallet::verify(
        &wallet.verifying_key,
        fake_message,
        &signature,
    );

    println!(
        "Modified transaction valid: {}",
        fake_valid
    );

    println!("=================================");
}
