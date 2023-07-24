use crate::Result;
use chrono::Utc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;

const DIFFICULTY_PREFIX: &str = "00";

type Data = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub id: u64,
    pub nonce: u64,
    pub timestamp: i64,
    pub data: Data,
    pub previous_hash: String,
    pub hash: String,
}

impl Default for Block {
    fn default() -> Self {
        let id: u64 = 0;
        let timestamp: i64 = 1_000_000_000;
        let data = String::from("genesis");
        let previous_hash = format!("{}00", DIFFICULTY_PREFIX);
        let (nonce, hash) = mine_block(id, timestamp, &data, previous_hash.as_str());

        Self {
            id,
            nonce,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

impl Block {
    pub fn new(id: u64, data: Data, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let (nonce, hash) = mine_block(id, timestamp, &data, &previous_hash);
        Self {
            id,
            nonce,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
    fn is_valid(&self, previous_block: &Block) -> bool {
        if self.previous_hash != previous_block.hash {
            log::warn!("block with id: {} has wrong previous hash", self.id);
            return false;
        } else if !hash2binary(&hex::decode(&self.hash).expect("can decode from hex"))
            .starts_with(DIFFICULTY_PREFIX)
        {
            log::warn!("block with id: {} has invalid difficulty", self.id);
            return false;
        } else if self.id != previous_block.id + 1 {
            log::warn!(
                "block with id: {} is not the next block after the latest: {}",
                self.id,
                previous_block.id
            );
            return false;
        } else if hex::encode(calculate_hash(
            self.id,
            self.nonce,
            self.timestamp,
            &self.data,
            &self.previous_hash,
        )) != self.hash
        {
            log::warn!("block with id: {} has invalid hash", self.id);
            return false;
        }
        true
    }
}

#[derive(PartialEq, Eq)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            blocks: vec![Block::default()],
        }
    }
}

impl PartialOrd for Chain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.is_valid(), other.is_valid()) {
            (false, false) => None,
            (false, true) => Some(Ordering::Less),
            (true, false) => Some(Ordering::Greater),
            (true, true) => Some(self.blocks.len().cmp(&other.blocks.len())),
        }
    }
}

impl Chain {
    pub fn try_add_block(&mut self, block: Block) {
        let previous_block = self.blocks.last().expect("there is at least one block");
        if block.is_valid(previous_block) {
            self.blocks.push(block);
        } else {
            log::error!("could not add block - invalid");
        }
    }
    pub fn add_data(&mut self, data: Data) -> Result<Block> {
        let Block { id, hash, .. } = self.blocks.last().expect("has to exist");
        let new_block = Block::new(id + 1, data, hash.to_owned());
        self.blocks.push(new_block.to_owned());
        Ok(new_block)
    }
    pub fn is_valid(&self) -> bool {
        if self.blocks.len() < 2 {
            return true;
        }
        // chain.windows(2).all(|bs| bs[1].is_valid(&bs[0]))
        self.blocks
            .iter()
            .tuple_windows::<(&Block, &Block)>()
            .all(|(b1, b2)| b2.is_valid(b1))
    }
    pub fn choose_chain(&mut self, remote: &Chain) {
        let Some(cmp) = (*self).partial_cmp(remote) else {
            panic!("both chains are invalid");
        };
        if matches!(cmp, Ordering::Less) {
            self.blocks = remote.blocks.to_owned();
        }
    }
}

fn calculate_hash(id: u64, nonce: u64, timestamp: i64, data: &str, previous_hash: &str) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn mine_block(id: u64, timestamp: i64, data: &Data, previous_hash: &str) -> (u64, String) {
    log::info!("mining block ...");
    let mut nonce = 0;

    loop {
        if nonce % 100_000 == 0 {
            log::info!("nonce: {}", nonce);
        }
        let hash = calculate_hash(id, nonce, timestamp, data.as_str(), previous_hash);
        let binary_hash = hash2binary(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            log::info!(
                "mined! nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

fn hash2binary(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
