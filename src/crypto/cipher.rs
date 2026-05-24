// Desenvolvido por L. A. Leandro São José dos Campos- SP - 23/05/2026

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};

pub fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

pub fn encrypt_chunk(key: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .expect("falha na criptografia AES-256-GCM");

    (nonce_bytes.to_vec(), ciphertext)
}

pub fn decrypt_chunk(
    key: &[u8; 32],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    cipher.decrypt(nonce, ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip_small() {
        let key = derive_key("teste-criptografia");
        let plaintext = b"Mensagem secreta: AES-256-GCM funcionando!";

        let (nonce, encrypted) = encrypt_chunk(&key, plaintext);
        let decrypted = decrypt_chunk(&key, &nonce, &encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key_correct = derive_key("senha-correta");
        let key_wrong = derive_key("senha-errada");
        let plaintext = b"Dados confidenciais";

        let (nonce, encrypted) = encrypt_chunk(&key_correct, plaintext);
        let result = decrypt_chunk(&key_wrong, &nonce, &encrypted);

        assert!(
            result.is_err(),
            "Descriptografia com chave errada deve falhar"
        );
    }

    #[test]
    fn test_empty_chunk() {
        let key = derive_key("chave-teste");
        let plaintext = b"";

        let (nonce, encrypted) = encrypt_chunk(&key, plaintext);
        let decrypted = decrypt_chunk(&key, &nonce, &encrypted).unwrap();

        assert!(decrypted.is_empty());
    }

    #[test]
    fn test_large_chunk() {
        let key = derive_key("chave-grande");
        let plaintext = vec![0xABu8; 1_048_576];

        let (nonce, encrypted) = encrypt_chunk(&key, &plaintext);
        let decrypted = decrypt_chunk(&key, &nonce, &encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_multiple_chunks_independence() {
        let key = derive_key("multi-chunk");
        let plaintexts = vec![
            b"Primeiro bloco" as &[u8],
            b"Segundo bloco com dados sensiveis",
            b"Terceiro",
            b"",
        ];

        let encrypted: Vec<_> = plaintexts
            .iter()
            .map(|p| encrypt_chunk(&key, p))
            .collect();

        for (i, (nonce, ciphertext)) in encrypted.iter().enumerate() {
            let decrypted = decrypt_chunk(&key, nonce, ciphertext).unwrap();
            assert_eq!(decrypted, plaintexts[i], "Falha no bloco {}", i);
        }
    }
}
