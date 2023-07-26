pub mod blocks;
pub mod encryption;
pub mod p2p;
pub mod utils_crypto;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
