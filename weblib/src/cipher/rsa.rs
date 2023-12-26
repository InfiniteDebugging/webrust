use base64::{engine::general_purpose, Engine as _};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use std::error::Error;
const KEY_SIZE: usize = 2048;

pub struct Rsa {
    data: String,
    private_key: RsaPrivateKey,
}

impl Rsa {
    pub fn new(input: String) -> Result<Self, Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, KEY_SIZE)?;
        let public_key = private_key.to_public_key();
        let input_bytes = input.as_bytes();
        let encrypted_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, input_bytes)?;
        let encoded_data = general_purpose::STANDARD.encode(encrypted_data);
        Ok(Self {
            data: encoded_data,
            private_key,
        })
    }
}

impl super::Cipher for Rsa {
    fn original_string(&self) -> Result<String, Box<dyn Error>> {
        let decoded_data = general_purpose::STANDARD.decode(&self.data)?;
        let decrypted_data = self.private_key.decrypt(Pkcs1v15Encrypt, &decoded_data)?;
        Ok(String::from_utf8(decrypted_data).expect("Issue while decoding"))
    }

    fn encrypted_string(&self) -> Result<String, Box<dyn Error>> {
        // Data is already encrypted.
        Ok(String::from(&self.data))
    }
}

#[cfg(test)]
mod test {
    use crate::cipher::Cipher;

    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let msg = String::from("value");
        let rsa_encryptor = Rsa::new(msg.clone()).expect("Something is wrong");
        assert_eq!(
            msg,
            rsa_encryptor
                .original_string()
                .expect("Issue getting original string")
        );
    }
}
