#[macro_use]
extern crate lazy_static;

mod board;
mod tile;

fn main() {
    let mut is_first_tile = true;
    for tile in tile::ALL_TILES.iter() {
        if !is_first_tile {
            println!("==");
        } else {
            is_first_tile = false;
        }
        let mut is_first_layout = true;
        for layout in tile.get_layouts() {
            if !is_first_layout {
                println!("-");
            } else {
                is_first_layout = false;
            }
            for y in 0..tile::MAX_SIZE {
                print!("    ");
                for x in 0..tile::MAX_SIZE {
                    let symbol = if layout.is_present_at(x, y) {
                        'X'
                    } else {
                        '·'
                    };
                    print!("{}", symbol);
                }
                println!();
            }
        }
    }
    let b = board::Board::all_blocked();
    println!("{:?} is all_blocked {}", &b, b.is_all_blocked());
}
