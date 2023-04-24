// wasm-pack build --target web

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate static_assertions;

use wasm_bindgen::prelude::*;

mod board;
mod search;
mod tile;

pub const VERSION: usize = 42;

#[wasm_bindgen]
pub fn check_config(
    version: usize,
    max_board_w: u8,
    max_board_h: u8,
    max_tile_size: u8,
    total_tiles: usize,
) -> u32 {
    if version != VERSION {
        return 1;
    }
    if max_board_w != board::MAX_WIDTH {
        return 2;
    }
    if max_board_h != board::MAX_HEIGHT {
        return 3;
    }
    if max_tile_size != tile::MAX_SIZE {
        return 4;
    }
    if total_tiles != tile::ALL_TILES.len() {
        return 5;
    }

    // Random number that is unlikely to have been generated by accident:
    134250805
}

const CELL_TO_TILE_LENGTH: usize = (board::MAX_WIDTH * board::MAX_HEIGHT) as usize;

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct Result {
    steps_taken: usize,
    has_solution: bool,
    has_finished: bool,
    cell_to_tile: [u8; CELL_TO_TILE_LENGTH],
}

fn decode_tile_indices(tiles_encoded: u32) -> Vec<usize> {
    let mut tile_indices = Vec::new();
    for tile_index in 0..tile::ALL_TILES.len() {
        if 0 != tiles_encoded & (1 << (tile::ALL_TILES.len() - 1 - tile_index)) {
            tile_indices.push(tile_index);
        }
    }
    tile_indices
}

fn paint_cells(steps: &search::Result, tile_lookup: &[usize]) -> [u8; CELL_TO_TILE_LENGTH] {
    let mut cells = [255; CELL_TO_TILE_LENGTH];
    for operation in steps {
        // Note that the 'tile_index' refers to the index in the '&[Tile]' given to
        // 'State::new()'. We need to translate that to the index in 'tile::ALL_TILES'.
        let local_tile_index = operation.indexed_tile_layout.tile_index;
        let global_tile_index = tile_lookup[local_tile_index as usize];
        let tile = &tile::ALL_TILES[global_tile_index];
        let layout = &tile.get_layouts()[operation.indexed_tile_layout.layout_index as usize];
        for y in 0..tile::MAX_SIZE {
            for x in 0..tile::MAX_SIZE {
                if !layout.is_present_at(x, y) {
                    continue;
                }
                let cell_index = (x + operation.dx) + board::MAX_WIDTH * (y + operation.dy);
                cells[cell_index as usize] = global_tile_index as u8;
            }
        }
    }
    cells
}

#[wasm_bindgen]
pub fn compute_result(tiles_encoded: u32, board_encoded: u32, max_steps: usize) -> Result {
    let tile_indices = decode_tile_indices(tiles_encoded);
    let tiles = tile_indices
        .iter()
        .map(|&i| tile::ALL_TILES[i].clone())
        .collect::<Vec<_>>();
    let board = board::Board::from_encoded(board_encoded);
    let mut search_state = search::State::new(board, &tiles);
    let (steps_taken, raw_result) = search_state.step_at_most(max_steps);
    let (has_solution, cell_to_tile) = match raw_result {
        None => (false, [255; CELL_TO_TILE_LENGTH]),
        Some(steps) => (true, paint_cells(&steps, &tile_indices)),
    };
    Result {
        steps_taken,
        has_solution,
        has_finished: !search_state.can_step(),
        cell_to_tile,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_tiles() {
        assert_eq!(decode_tile_indices(0x000), vec![]);
        assert_eq!(decode_tile_indices(0x001), vec![11]);
        assert_eq!(decode_tile_indices(0x002), vec![10]);
        assert_eq!(decode_tile_indices(0x004), vec![9]);
        assert_eq!(decode_tile_indices(0x008), vec![8]);
        assert_eq!(decode_tile_indices(0x010), vec![7]);
        assert_eq!(decode_tile_indices(0x020), vec![6]);
        assert_eq!(decode_tile_indices(0x040), vec![5]);
        assert_eq!(decode_tile_indices(0x080), vec![4]);
        assert_eq!(decode_tile_indices(0x100), vec![3]);
        assert_eq!(decode_tile_indices(0x200), vec![2]);
        assert_eq!(decode_tile_indices(0x400), vec![1]);
        assert_eq!(decode_tile_indices(0x800), vec![0]);
        assert_eq!(decode_tile_indices(0x123), vec![3, 6, 10, 11]);
        assert_eq!(
            decode_tile_indices(0xFFF),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        );
    }

    fn make_result(
        steps_taken: usize,
        has_solution: bool,
        has_finished: bool,
        cell_to_tile: [u8; CELL_TO_TILE_LENGTH],
    ) -> Result {
        Result {
            steps_taken,
            has_solution,
            has_finished,
            cell_to_tile,
        }
    }

    #[test]
    fn test_simple_positive() {
        #[rustfmt::skip]
        assert_eq!(
            // LSB ·XX··
            //     XXXX·
            //     ·XXXX
            //     ·XXX·
            //     ·····
            //     ····· MSB
            compute_result(0x062, 0x000779E6, 100),
            make_result(
                22,
                true,
                false,
                [
                    255, 0xA, 0x5, 255, 255,
                    0xA, 0xA, 0x5, 0x5, 255,
                    255, 0xA, 0x5, 0x6, 0x6,
                    255, 0xA, 0x6, 0x6, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                ]
            )
        );
    }

    #[test]
    fn test_simple_negative_impossible() {
        #[rustfmt::skip]
        assert_eq!(
            // LSB X·X··
            //     XXXX·
            //     ·XXXX
            //     ·XXX·
            //     ·····
            //     ····· MSB
            compute_result(0x062, 0x000779E5, 100),
            make_result(
                17,
                false,
                true,
                [
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                ]
            )
        );
    }

    #[test]
    fn test_simple_negative_timeout() {
        #[rustfmt::skip]
        assert_eq!(
            // LSB ·XX··
            //     XXXX·
            //     ·XXXX
            //     ·XXX·
            //     ·····
            //     ····· MSB
            compute_result(0x062, 0x000779E6, 10),
            make_result(
                10,
                false,
                false,
                [
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                    255, 255, 255, 255, 255,
                ]
            )
        );
    }
}
