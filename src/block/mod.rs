extern crate time;
extern crate sha2;

#[derive(Debug)]
pub struct Block {
    index: u64,
    timestamp: i64,
    hash: String,
    previous_hash: String,
    data: String,
}

pub fn create_genesis_block() -> Block {
    let genesis_block = Block {
        index: 0,
        timestamp: time::now().to_timespec().sec,
        hash: String::from("hash"),
        previous_hash: String::from(""),
        data: "First block!".to_string(),
    };

    genesis_block
}

pub fn mine() -> Block {

}

