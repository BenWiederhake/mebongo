use crate::tile;

// This can be changed easily, as long as MAX_WIDTH * MAX_HEIGHT <= 64.
pub const MAX_WIDTH: u8 = 5;
pub const MAX_HEIGHT: u8 = 5;

type BitType = u32;

#[derive(Clone, Debug)]
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

    pub fn set_blocked(&mut self, x: u8, y: u8) {
        self.bit_data &= !Self::index_mask(x, y);
    }

    pub fn set_unblocked(&mut self, x: u8, y: u8) {
        self.bit_data |= Self::index_mask(x, y);
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
}
