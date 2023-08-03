use std::fmt;
use chrono::prelude::*;
use sha2::{Sha256, Digest};

// Define the Block structure
struct Block {
    index: u32,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, timestamp: i64, data: String, prev_hash: String) -> Block {
        let hash = Block::calculate_hash(index, timestamp, &data, &prev_hash);
        Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }

    fn calculate_hash(index: u32, timestamp: i64, data: &str, prev_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Index: {}\nTimestamp: {}\nData: {}\nPrevious Hash: {}\nHash: {}\n", 
               self.index, self.timestamp, self.data, self.prev_hash, self.hash)
    }
}

// Define the Blockchain structure
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, Utc::now().timestamp(), String::from("Genesis Block"), String::from("0"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u32;
        let timestamp = Utc::now().timestamp();
        let new_block = Block::new(index, timestamp, data, prev_hash);
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if i > 0 && block.hash != Block::calculate_hash(block.index, block.timestamp, &block.data, &block.prev_hash) {
                return false;
            }
            if i > 0 && block.prev_hash != self.chain[i - 1].hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(String::from("First Block"));
    blockchain.add_block(String::from("Second Block"));

    for block in blockchain.chain {
        println!("{}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());
}