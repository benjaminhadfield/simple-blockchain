extern crate time;
extern crate sha3;

use self::sha3::{Digest, Sha3_256};
use generic_array::{GenericArray};
use generic_array::typenum::{U64};

#[derive(Debug)]
pub struct Block {
    hash: GenericArray<u8, U64>,
    previous_hash: GenericArray<u8, U64>,
    index: u64,
    timestamp: i64,
    data: String,
}

impl Block {
    fn new(&self, blockchain: &Vec<Block>, data: String) -> Block {
        // Get details of the last block.
        let last_block = blockchain.last().unwrap();
        let index = last_block.index + 1;
        let previous_hash = last_block.hash;
        // Get the current time.
        // For this simple implementation, sec granularity is fine.
        let timestamp = time::now_utc().to_timespec().sec;
        // Create the new hash.
        let mut hasher = Sha3_256::default();
        hasher.input(previous_hash);
        hasher.input(index.to_string().as_bytes());
        hasher.input(timestamp.to_string().as_bytes());
        hasher.input(data.as_bytes());
        let hash = hasher.result();

        Block {
            hash: hash,
            previous_hash: previous_hash,
            index: index,
            timestamp: timestamp,
            data: data,
        }
    }
}

pub fn create_genesis_block() -> Block {
    let genesis_block = Block {
        index: 0,
        timestamp: time::now_utc().to_timespec().sec,
        hash: arr![U64; 0, 3],
        previous_hash: arr![U64; 0, 3],
        data: "First block!".to_string(),
    };

    genesis_block
}

// pub fn mine() -> Block {

// }

