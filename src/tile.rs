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

    #[cfg(test)]
    pub fn new_for_test(layouts_raw: Vec<u16>) -> Tile {
        Tile::new(layouts_raw)
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

    #[cfg(test)]
    pub fn new_for_test(bit_data: u16) -> TileLayout {
        TileLayout { bit_data }
    }
}

lazy_static! {
    pub static ref ALL_TILES: Vec<Tile> = vec![
        /*
         * Bit order:
         * 0 1 2 3
         * 4 5 6 7
         * 8 9 A B
         * C D E F
         * … where 0 is the LSB and F is the MSB of the underlying u16.
         */
        /*
         * XX·· 3
         * XX·· 3
         * ···· 0
         * ···· 0
         */
        Tile::new(vec![0x0033]),
        /*
         * XX·· 3 X··· 1
         * ···· 0 X··· 1
         * ···· 0 ···· 0
         * ···· 0 ···· 0
         */
        Tile::new(vec![0x0003, 0x0011]),
        /*
         * XXX· 7 X··· 1
         * ···· 0 X··· 1
         * ···· 0 X··· 1
         * ···· 0 ···· 0
         */
        Tile::new(vec![0x0007, 0x0111]),
        /*
         * XXXX F X··· 1
         * ···· 0 X··· 1
         * ···· 0 X··· 1
         * ···· 0 X··· 1
         */
        Tile::new(vec![0x000F, 0x1111]),
        /*
         * XX·· 3 XX·· 3 ·X·· 2 X··· 1
         * X··· 1 ·X·· 2 XX·· 3 XX·· 3
         * ···· 0 ···· 0 ···· 0 ···· 0
         * ···· 0 ···· 0 ···· 0 ···· 0
         */
        Tile::new(vec![0x0013, 0x0023, 0x0032, 0x0031]),
        /*
         * ·X·· 2 X··· 1 XXX· 7 ·X·· 2
         * XXX· 7 XX·· 3 ·X·· 2 XX·· 3
         * ···· 0 X··· 1 ···· 0 ·X·· 2
         * ···· 0 ···· 0 ···· 0 ···· 0
         */
        Tile::new(vec![0x0072, 0x0131, 0x0027, 0x0232]),
        /*
         * XX·· 3 ·XX· 6 X··· 1 ·X·· 2
         * ·XX· 6 XX·· 3 XX·· 3 XX·· 3
         * ···· 0 ···· 0 ·X·· 2 X··· 1
         * ···· 0 ···· 0 ···· 0 ···· 0
         */
        Tile::new(vec![0x0063, 0x0036, 0x0231, 0x0132]),
        /*
         * XX·· 3 ·XX· 6 X··· 1 ··X· 4
         * ·X·· 2 ·X·· 2 XXX· 7 XXX· 7
         * ·XX· 6 XX·· 3 ··X· 4 X··· 1
         * ···· 0 ···· 0 ···· 0 ···· 0
         */
        Tile::new(vec![0x0623, 0x0326, 0x0471, 0x0174]),
        /*
         * XX·· 3 XXX· 7 ·X·· 2 X··· 1 XX·· 3 XXX· 7 X··· 1 ··X· 4
         * X··· 1 ··X· 4 ·X·· 2 XXX· 7 ·X·· 2 X··· 1 X··· 1 XXX· 7
         * X··· 1 ···· 0 XX·· 3 ···· 0 ·X·· 2 ···· 0 XX·· 3 ···· 0
         * ···· 0 ···· 0 ···· 0 ···· 0 ···· 0 ···· 0 ···· 0 ···· 0
         */
        Tile::new(vec![0x0113, 0x0047, 0x0322, 0x0071, 0x0223, 0x0017, 0x0311, 0x0074]),
        /*
         * XX·· 3 XXXX F ·X·· 2 X··· 1 XX·· 3 XXXX F X··· 1 ···X 8
         * X··· 1 ···X 8 ·X·· 2 XXXX F ·X·· 2 X··· 1 X··· 1 XXXX F
         * X··· 1 ···· 0 ·X·· 2 ···· 0 ·X·· 2 ···· 0 X··· 1 ···· 0
         * X··· 1 ···· 0 XX·· 3 ···· 0 ·X·· 2 ···· 0 XX·· 3 ···· 0
         */
        Tile::new(vec![0x1113, 0x008F, 0x3222, 0x00F1, 0x2223, 0x001F, 0x3111, 0x00F8]),
        /*
         * X··· 1 XXXX F ·X·· 2 ·X·· 2 ·X·· 2 XXXX F X··· 1 ··X· 4
         * XX·· 3 ··X· 4 ·X·· 2 XXXX F XX·· 3 ·X·· 2 X··· 1 XXXX F
         * X··· 1 ···· 0 XX·· 3 ···· 0 ·X·· 2 ···· 0 XX·· 3 ···· 0
         * X··· 1 ···· 0 ·X·· 2 ···· 0 ·X·· 2 ···· 0 X··· 1 ···· 0
         */
        Tile::new(vec![0x1131, 0x004F, 0x2322, 0x00F2, 0x2232, 0x002F, 0x1311, 0x00F4]),
        /*
         * XX·· 3 XX·· 3 ·X·· 2 XXX· 7 XX·· 3 XXX· 7 X··· 1 ·XX· 6
         * XX·· 3 XXX· 7 XX·· 3 ·XX· 6 XX·· 3 XX·· 3 XX·· 3 XXX· 7
         * X··· 1 ···· 0 XX·· 3 ···· 0 ·X·· 2 ···· 0 XX·· 3 ···· 0
         * ···· 0 ···· 0 ···· 0 ···· 0 ···· 0 ···· 0 ···· 0 ···· 0
         */
        Tile::new(vec![0x0133, 0x0073, 0x0332, 0x0067, 0x0233, 0x0037, 0x0331, 0x0076]),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_layouts_are_unique() {
        let layout_bits = ALL_TILES
            .iter()
            .flat_map(|t| t.get_layouts())
            .map(|l| l.bit_data)
            .collect::<Vec<_>>();
        println!("{:?}", layout_bits);
        let layout_bits_dedup = layout_bits.iter().copied().collect::<HashSet<_>>();
        println!("{:?}", layout_bits_dedup);
        assert_eq!(layout_bits.len(), layout_bits_dedup.len());
    }
}
