extern crate implementation;

fn main() {
   let gen_block = implementation::block::create_genesis_block();
   let bcn = implementation::blockchain::create_blockchain(gen_block);
   println!("bcn: {:?}", bcn);
}
