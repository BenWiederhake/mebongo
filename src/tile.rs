// This is assumed to be 4 in several places
pub const MAX_SIZE: u8 = 4;

#[derive(Debug)]
pub struct Tile {
    layouts: Vec<TileLayout>,
}

impl Tile {
    fn new(layouts_raw: Vec<u16>) -> Tile {
        Tile {
            layouts: layouts_raw
                .into_iter()
                .map(|bit_data| TileLayout { bit_data })
                .collect(),
        }
    }

    pub fn get_layouts(&self) -> &[TileLayout] {
        &self.layouts
    }
}

#[derive(Debug)]
pub struct TileLayout {
    bit_data: u16,
}

impl TileLayout {
    pub fn is_present_at(&self, x: u8, y: u8) -> bool {
        assert!(x < MAX_SIZE && y < MAX_SIZE);
        let offset = x + y * MAX_SIZE;
        0 != (self.bit_data & (1 << offset))
    }
}

lazy_static! {
    pub static ref ALL_TILES: Vec<Tile> = vec![
        Tile::new(vec![0, 1, 2, 3]),
        Tile::new(vec![4, 5]),
        Tile::new(vec![42]),
    ];
}
