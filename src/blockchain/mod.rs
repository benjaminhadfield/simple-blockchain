use block::Block;

pub fn create_blockchain(genesis_block: Block) -> Vec<Block> {
  vec![genesis_block]
}
