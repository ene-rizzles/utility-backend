use ed25519_dalek::{Signature, Verifier, VerifyingKey};

pub struct MeterIdentity {
    pub meter_id: String,
    pub public_key: VerifyingKey,
}

pub fn verify_packet(
    identity: &MeterIdentity,
    payload: &[u8],
    signature: &[u8],
) -> Result<(), &'static str> {
    let sig = Signature::from_slice(signature).map_err(|_| "invalid signature format")?;
    identity
        .public_key
        .verify(payload, &sig)
        .map_err(|_| "cryptographic signature mismatch")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;

    #[test]
    fn test_sign_verify() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let identity = MeterIdentity {
            meter_id: "MTR-007".into(),
            public_key: verifying_key,
        };
        let payload = b"voltage:240.1;current:15.3";
        let signature = signing_key.sign(payload);
        assert!(verify_packet(&identity, payload, &signature.to_bytes()).is_ok());
    }
}
