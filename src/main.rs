#[macro_use]
extern crate lazy_static;

mod tile;

fn main() {
    println!("Hello, world! -> {:?}", *tile::ALL_TILES);
}
