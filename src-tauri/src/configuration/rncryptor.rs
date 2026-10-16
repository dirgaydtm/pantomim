use aes::cipher::{BlockModeDecrypt, KeyIvInit, block_padding::Pkcs7};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

use super::Error;

const HEADER_LEN: usize = 2 + 8 + 8 + 16;
const HMAC_LEN: usize = 32;

/// Decrypts an RNCryptor password-based message.
///
/// Version and options bytes are not checked: SEB files carry version 2, whose layout equals v3.
pub(super) fn decrypt(block: &[u8], password: &str) -> Result<Vec<u8>, Error> {
    if block.len() < HEADER_LEN + HMAC_LEN {
        return Err(Error::InvalidData("encrypted block too short".into()));
    }
    let (signed, tag) = block.split_at(block.len() - HMAC_LEN);
    let encryption_salt = &block[2..10];
    let hmac_salt = &block[10..18];
    let iv = &block[18..HEADER_LEN];
    let ciphertext = &signed[HEADER_LEN..];
    let derive = |salt: &[u8]| {
        pbkdf2::pbkdf2_hmac_array::<sha1::Sha1, 32>(password.as_bytes(), salt, 10_000)
    };

    let mut mac =
        Hmac::<Sha256>::new_from_slice(&derive(hmac_salt)).expect("HMAC accepts any key length");
    mac.update(signed);
    mac.verify_slice(tag).map_err(|_| Error::WrongPassword)?;

    cbc::Decryptor::<aes::Aes256>::new_from_slices(&derive(encryption_salt), iv)
        .expect("key and IV lengths are fixed")
        .decrypt_padded_vec::<Pkcs7>(ciphertext)
        .map_err(|_| Error::InvalidData("bad padding".into()))
}
