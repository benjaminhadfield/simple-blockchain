extern crate implementation;

fn main() {
   let b = implementation::block::create_genesis_block(); 
   println!("b is {:?}", b);
}

