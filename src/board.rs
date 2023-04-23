use crate::tile;

// This can be changed easily, as long as MAX_WIDTH * MAX_HEIGHT <= 64.
pub const MAX_WIDTH: u8 = 5;
pub const MAX_HEIGHT: u8 = 6;
type BitType = u32;
const_assert!(BitType::BITS > (MAX_WIDTH * MAX_HEIGHT) as u32);

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    bit_data: BitType,
}

impl Board {
    pub fn all_blocked() -> Board {
        Board { bit_data: 0 }
    }

    pub fn is_all_blocked(&self) -> bool {
        self.bit_data == 0
    }

    fn index_mask(x: u8, y: u8) -> BitType {
        assert!(x < MAX_WIDTH && y < MAX_HEIGHT);
        let index = x + MAX_WIDTH * y;
        assert!(u32::from(index) < BitType::BITS); // Shifting a 1u32 by 32 bits does something other than you think!
        1 << index
    }

    #[cfg(test)]
    pub fn index_mask_for_test(x: u8, y: u8) -> BitType {
        Self::index_mask(x, y)
    }

    pub fn is_blocked_at(&self, x: u8, y: u8) -> bool {
        0 == (self.bit_data & Self::index_mask(x, y))
    }

    pub fn set_blocked(&mut self, x: u8, y: u8) {
        self.bit_data &= !Self::index_mask(x, y);
    }

    pub fn set_unblocked(&mut self, x: u8, y: u8) {
        self.bit_data |= Self::index_mask(x, y);
    }

    pub fn with_blocked_tile(
        &self,
        tile_layout: &tile::TileLayout,
        dx: u8,
        dy: u8,
    ) -> Option<Board> {
        assert!(dx < MAX_WIDTH && dy < MAX_HEIGHT);
        let mut result = self.clone();
        for y in 0..tile::MAX_SIZE {
            for x in 0..tile::MAX_SIZE {
                if !tile_layout.is_present_at(x, y) {
                    continue;
                }
                // Try to remove from that position:
                let abs_x = dx + x;
                let abs_y = dy + y;
                if abs_x >= MAX_WIDTH || abs_y >= MAX_HEIGHT || self.is_blocked_at(abs_x, abs_y) {
                    // Impossible, abort.
                    return None;
                }
                result.set_blocked(abs_x, abs_y);
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_all_blocked() {
        let b = Board::all_blocked();
        assert_eq!(b.bit_data, 0);
        assert!(b.is_all_blocked());
    }

    #[test]
    fn test_basic_blocked_idempotency() {
        let mut b = Board::all_blocked();
        b.set_blocked(0, 0);
        assert_eq!(b.bit_data, 0);
        assert!(b.is_all_blocked());
    }

    #[test]
    fn test_basic_set_unblocked() {
        let mut b = Board::all_blocked();
        b.set_unblocked(0, 0);
        assert_eq!(b.bit_data, 1);
        assert!(!b.is_all_blocked());
        b.set_unblocked(0, 2);
        // This assumes MAX_WIDTH == 5:
        let mask_0_2 = 0x400;
        assert_eq!(b.bit_data, 1 + mask_0_2);
        assert!(!b.is_all_blocked());
        // Test idempotency
        b.set_unblocked(0, 2);
        assert_eq!(b.bit_data, 1 + mask_0_2);
        assert!(!b.is_all_blocked());
        // Test set_blocked
        b.set_blocked(0, 0);
        assert_eq!(b.bit_data, mask_0_2);
        assert!(!b.is_all_blocked());
    }

    #[test]
    fn test_index_mask() {
        assert_eq!(Board::index_mask_for_test(0, 0), 0x00000001);
        assert_eq!(Board::index_mask_for_test(1, 0), 0x00000002);
        assert_eq!(Board::index_mask_for_test(2, 0), 0x00000004);
        assert_eq!(Board::index_mask_for_test(3, 0), 0x00000008);
        assert_eq!(Board::index_mask_for_test(4, 0), 0x00000010);
        assert_eq!(Board::index_mask_for_test(0, 1), 0x00000020);
        assert_eq!(Board::index_mask_for_test(1, 1), 0x00000040);
        assert_eq!(Board::index_mask_for_test(2, 1), 0x00000080);
        assert_eq!(Board::index_mask_for_test(3, 1), 0x00000100);
        assert_eq!(Board::index_mask_for_test(4, 1), 0x00000200);
        assert_eq!(Board::index_mask_for_test(0, 2), 0x00000400);
        assert_eq!(Board::index_mask_for_test(0, 3), 0x00008000);
        assert_eq!(Board::index_mask_for_test(0, 4), 0x00100000);
        assert_eq!(Board::index_mask_for_test(1, 4), 0x00200000);
        assert_eq!(Board::index_mask_for_test(2, 4), 0x00400000);
        assert_eq!(Board::index_mask_for_test(3, 4), 0x00800000);
        assert_eq!(Board::index_mask_for_test(4, 4), 0x01000000);
    }

    fn tile_from(bits: u16) -> tile::TileLayout {
        tile::TileLayout::new_for_test(bits)
    }

    #[test]
    fn test_basic_blocked_tile_positive() {
        let mut b = Board::all_blocked();
        b.set_unblocked(0, 0);
        assert_eq!(b.bit_data, 1);
        assert_eq!(
            b.with_blocked_tile(&tile_from(0x0001), 0, 0),
            Some(Board::all_blocked())
        );
    }

    #[test]
    fn test_empty_blocked_tile_positive() {
        let b = Board::all_blocked();
        assert_eq!(b.bit_data, 0);
        assert_eq!(
            b.with_blocked_tile(&tile_from(0x0000), 0, 0),
            Some(Board::all_blocked())
        );
        assert_eq!(
            b.with_blocked_tile(&tile_from(0x0000), 1, 0),
            Some(Board::all_blocked())
        );
        assert_eq!(
            b.with_blocked_tile(&tile_from(0x0000), 3, 3),
            Some(Board::all_blocked())
        );
    }

    #[test]
    fn test_basic_blocked_tile_negative() {
        let mut b = Board::all_blocked();
        b.set_unblocked(0, 0);
        assert_eq!(b.bit_data, 1);
        assert_eq!(b.with_blocked_tile(&tile_from(0x0001), 0, 1), None);
        assert_eq!(b.with_blocked_tile(&tile_from(0x0001), 3, 3), None);
        assert_eq!(b.with_blocked_tile(&tile_from(0x0001), 1, 0), None);
    }

    #[test]
    fn test_two_blocked_tile() {
        let mut b = Board::all_blocked();
        b.set_unblocked(0, 1);
        b.set_unblocked(0, 2);
        assert_eq!(b.with_blocked_tile(&tile_from(0x0011), 0, 0), None);
        assert_eq!(
            b.with_blocked_tile(&tile_from(0x0011), 0, 1),
            Some(Board::all_blocked())
        );
        assert_eq!(b.with_blocked_tile(&tile_from(0x0011), 0, 2), None);
    }
}
