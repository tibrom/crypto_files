use aes::Aes256;
use cipher::{KeyIvInit, StreamCipher};
use ctr::Ctr128BE;
use rand::RngCore;

use crate::domain::models::Credentials;
use crate::domain::services::{ConsoleError, CryptoService};

type Aes256Ctr = Ctr128BE<Aes256>;

const MAGIC: &[u8] = b"CRYPT1";

#[derive(Debug)]
pub enum CryptoError {
    InvalidHeader(String),
    AesError(String),
}

impl ConsoleError for CryptoError {
    fn consol_log(&self) -> String {
        match self {
            CryptoError::InvalidHeader(e) => format!("Некорректный заголовок: {e}"),
            CryptoError::AesError(e) => format!("Ошибка AES: {e}"),
        }
    }
}

#[derive(Clone)]
pub struct AesCtrCryptoService {
    key: [u8; 32],
    iv: [u8; 16],
    cipher: Option<Aes256Ctr>,
    is_first_chunk: bool,
}

impl From<Credentials> for AesCtrCryptoService {
    fn from(config: Credentials) -> Self {
        let key_slice = config.key.as_slice();
        let mut key = [0u8; 32];
        let copy_len = key_slice.len().min(32);
        key[..copy_len].copy_from_slice(&key_slice[..copy_len]);

        Self {
            key,
            iv: [0u8; 16],
            cipher: None,
            is_first_chunk: true,
        }
    }
}

impl AesCtrCryptoService {
    fn create_iv() -> [u8; 16] {
        let mut iv = [0u8; 16];
        rand::rng().fill_bytes(&mut iv);
        iv
    }

    fn init_cipher(&mut self) {
        if self.cipher.is_none() {
            let c = Aes256Ctr::new(&self.key.into(), &self.iv.into());
            self.cipher = Some(c);
        }
    }

    fn apply_cipher_to(&mut self, data: &mut [u8]) -> Result<(), CryptoError> {
        self.init_cipher();
        if let Some(cipher) = self.cipher.as_mut() {
            cipher
                .try_apply_keystream(data)
                .map_err(|e| CryptoError::AesError(format!("{e:?}")))
        } else {
            Err(CryptoError::AesError("Cipher not initialized".into()))
        }
    }

    fn build_header_for_encrypt(&mut self) -> Vec<u8> {
        let mut header = Vec::with_capacity(MAGIC.len() + 16);
        header.extend_from_slice(MAGIC);
        header.extend_from_slice(&self.iv);
        header
    }
}

impl CryptoService for AesCtrCryptoService {
    type Error = CryptoError;

    fn is_encrypt(&mut self, chunk: &Vec<u8>) -> Result<bool, Self::Error> {
        Ok(chunk.starts_with(MAGIC))
    }

    fn encrypt(&mut self, mut chunk: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        if self.is_first_chunk {
            self.is_first_chunk = false;
            self.iv = Self::create_iv();
            self.cipher = None;
            self.init_cipher();

            self.apply_cipher_to(&mut chunk)?;

            let mut out = self.build_header_for_encrypt();
            out.extend_from_slice(&chunk);
            Ok(out)
        } else {
            self.apply_cipher_to(&mut chunk)?;
            Ok(chunk)
        }
    }

    fn decrypt(&mut self, mut chunk: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        if self.is_first_chunk {
            self.is_first_chunk = false;

            if !chunk.starts_with(MAGIC) {
                return Err(CryptoError::InvalidHeader(
                    "Файл не содержит MAGIC-токена".into(),
                ));
            }

            if chunk.len() < MAGIC.len() + 16 {
                return Err(CryptoError::InvalidHeader(
                    "Слишком короткий заголовок".into(),
                ));
            }

            self.iv
                .copy_from_slice(&chunk[MAGIC.len()..MAGIC.len() + 16]);

            self.cipher = None;
            self.init_cipher();

            let rest = chunk.split_off(MAGIC.len() + 16);
            let mut data = rest;
            if !data.is_empty() {
                self.apply_cipher_to(&mut data)?;
            }
            Ok(data)
        } else {
            self.apply_cipher_to(&mut chunk)?;
            Ok(chunk)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::Credentials;

    fn make_service() -> AesCtrCryptoService {
        let cred = Credentials {
            key: b"12345678901234567890123456789012".to_vec(),
            chunk_size: 16,
        };

        AesCtrCryptoService::from(cred)
    }

    #[test]
    fn test_encrypt_decrypt_chunk() {
        let mut enc = make_service();
        let mut dec = make_service();

        let chunk1 = b"Bla bla bla ".to_vec();
        let chunk2 = b"Bob Bob Bob".to_vec();

        let enc1 = enc.encrypt(chunk1.clone()).unwrap();
        let enc2 = enc.encrypt(chunk2.clone()).unwrap();

        let dec1 = dec.decrypt(enc1).unwrap();
        let dec2 = dec.decrypt(enc2).unwrap();

        assert_eq!(chunk1, dec1);
        assert_eq!(chunk2, dec2);
    }

    #[test]
    fn test_different_iv() {
        let mut s1 = make_service();
        let mut s2 = make_service();

        let data = b"AAAAA".to_vec();

        let e1 = s1.encrypt(data.clone()).unwrap();
        let e2 = s2.encrypt(data.clone()).unwrap();

        assert_ne!(e1, e2);
    }

    #[test]
    fn test_different_encypted_data() {
        let mut s1 = make_service();

        let data = b"AAAAA".to_vec();

        let e1 = s1.encrypt(data.clone()).unwrap();
        let e2 = s1.encrypt(data.clone()).unwrap();

        assert_ne!(e1, e2);
    }
}
