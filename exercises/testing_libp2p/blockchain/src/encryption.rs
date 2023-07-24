use crate::utils_crypto;

pub struct ExchangeBuilder {
    p: Option<u32>,
    g: Option<u32>,
    secret: Option<u32>,
    shared_key: Option<u32>,
}

impl ExchangeBuilder {
    fn new(p: Option<u32>) -> Self {
        let (p, g, secret) = match p {
            Some(other_public) => {
                // it's responding a handshake
                let public = utils_crypto::get_random_pr(other_public).unwrap();
                let secret = utils_crypto::gen_number(other_public);
                (public, Some(other_public), secret)
            }
            None => {
                // we are initializing the handshake
                let public = utils_crypto::gen_prime().try_into().unwrap();
                let secret = utils_crypto::gen_number(public);
                (public, None, secret)
            }
        };
        Self {
            p: Some(p),
            g,
            secret: Some(secret),
            shared_key: None,
        }
    }
    fn resp_handshake(&self, g: Option<u32>) -> u32 {
        let g = self.g.unwrap_or(g.unwrap());
        g.pow(self.secret.unwrap()) % self.p.unwrap()
    }
    fn get_public(&self) -> u32 {
        self.p.unwrap()
    }
    fn build(self, resp: Option<u32>) -> Exchange {
        todo!()
    }
}

pub struct Exchange {
    pub shared_key: u32,
}

impl Exchange {
    pub fn encrypt(&self, ciphertext: &str) -> String {
        ciphertext
            .chars()
            .map(|c| char::from_u32(c as u32 + self.shared_key).unwrap_or(c))
            .collect()
    }

    pub fn decrypt(&self, message: &str) -> String {
        message
            .chars()
            .map(|c| {
                let shifted_char = c as u32 - self.shared_key;
                char::from_u32(shifted_char).unwrap_or(c)
            })
            .collect()
    }
}

// Encryption function

// Decryption function

fn hex_ciphertext(m: &str) -> String {
    m.chars()
        .map(|c| format!("{:02x}", c as u32))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_handshake() {}

    #[test]
    fn safe_char_sum() {
        let max_char: u32 = char::MAX as u32;
        let max_key = u32::MAX - max_char;
        dbg!(max_key);
        assert!(matches!(max_key.checked_add(max_char), Some(_)));
    }

    #[test]
    fn basic_decription() {
        let exchange = Exchange { shared_key: 255 };
        let message = "Hi there! 😃";

        let ciphertext = exchange.encrypt(message);
        dbg!(hex_ciphertext(&ciphertext));
        let decrypted_msg = exchange.decrypt(&ciphertext);
        assert_eq!(message, decrypted_msg);
    }

    #[test]
    fn builder_exchange() {
        let alice_eb = ExchangeBuilder::new(None);
        let bob_eb = ExchangeBuilder::new(Some(alice_eb.get_public()));

        let b = bob_eb.resp_handshake(None);
        let a = alice_eb.resp_handshake(Some(b));

        let alice = alice_eb.build(None);
        let bob = bob_eb.build(Some(a));
        let message = "Hello bob! 😃";

        assert_eq!(alice.shared_key, bob.shared_key);
        assert_eq!(String::from(message), bob.decrypt(&alice.encrypt(message)));
        assert_eq!(String::from(message), alice.decrypt(&bob.encrypt(message)));
    }
}
