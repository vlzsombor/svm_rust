pub trait CryptoService {
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertext: &str) -> String;
}

pub struct AesCryptoService;

impl CryptoService for AesCryptoService {
    fn encrypt(&self, plaintext: &str) -> String {
        format!("encrypted: {}", plaintext)
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        ciphertext.replace("encrypted: ", "")
    }
}

pub struct MessageProcessor<T: CryptoService> {
    crypto_service: T,
}

impl<T: CryptoService> MessageProcessor<T> {
    pub fn new(crypto_service: T) -> Self {
        Self { crypto_service }
    }

    pub fn process_message(&self, input: &str) -> String {
        self.crypto_service.encrypt(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let crypto = AesCryptoService;
        let processor = MessageProcessor::new(crypto);
        let result = processor.process_message("test");
        assert_eq!(result, "encrypted: test");
    }
}