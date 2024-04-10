pub mod encryption {
    use rsa::{
        pkcs1v15::SigningKey,
        sha2::Sha256,
        signature::{RandomizedSigner, SignatureEncoding},
        Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    };

    const CLIENT_PUBLIC_KEY: &[u8] = include_bytes!("../client_public_key.der");
    const SERVER_PRIVATE_KEY: &[u8] = include_bytes!("../server_private_key.der");
    const RSA_CHUNK_SIZE: usize = 245;

    #[must_use]
    pub fn rsa_encrypt(data: &[u8]) -> Vec<u8> {
        let public_key: RsaPublicKey =
            rsa::pkcs8::DecodePublicKey::from_public_key_der(CLIENT_PUBLIC_KEY)
                .expect("Failed to read public key from der");
        let mut rng = rand::thread_rng();

        let mut result: Vec<u8> = Vec::new();
        for chunk in data.chunks(RSA_CHUNK_SIZE) {
            let encrypted_chunk = public_key
                .encrypt(&mut rng, Pkcs1v15Encrypt, chunk)
                .expect("Encryption failed!");

            result.extend(encrypted_chunk);
        }

        result
    }

    pub fn rsa_decrypt(cipher: &[u8]) -> Vec<u8> {
        let private_key: RsaPrivateKey =
            rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(SERVER_PRIVATE_KEY)
                .expect("Failed to read pkcs8 private key!");

        private_key.decrypt(Pkcs1v15Encrypt, cipher).unwrap()
    }

    #[must_use]
    pub fn rsa_sign(data: &[u8]) -> Box<[u8]> {
        let private_key = rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(SERVER_PRIVATE_KEY)
            .expect("Failed to read pkcs8 private key!");
        let signing_key = SigningKey::<Sha256>::new(private_key);

        let mut rng = rand::thread_rng();
        signing_key.sign_with_rng(&mut rng, data).to_bytes()
    }
}
