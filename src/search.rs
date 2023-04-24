use crate::board::{self, Board};
use crate::tile::Tile;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndexedTileLayout {
    pub tile_index: u8,
    pub layout_index: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    pub indexed_tile_layout: IndexedTileLayout,
    pub dx: u8,
    pub dy: u8,
}

impl Operation {
    fn from(tile_index: u8, layout_index: u8, dx: u8, dy: u8) -> Operation {
        Operation {
            indexed_tile_layout: IndexedTileLayout {
                tile_index,
                layout_index,
            },
            dx,
            dy,
        }
    }
}

#[derive(Debug)]
struct Node {
    board: Board,
    operation_and_parent_index: Option<(Operation, usize)>,
}

impl Node {
    fn new_root(board: Board) -> Node {
        Node {
            board,
            operation_and_parent_index: None,
        }
    }

    fn find_all_fits(&self, own_index: usize, tile: &Tile, tile_index: u8) -> Vec<Node> {
        let mut result = Vec::new();
        for (layout_index_us, layout) in tile.get_layouts().iter().enumerate() {
            let layout_index = layout_index_us as u8;
            for dy in 0..board::MAX_HEIGHT {
                for dx in 0..board::MAX_WIDTH {
                    if let Some(child_board) = self.board.with_blocked_tile(layout, dx, dy) {
                        let operation = Operation::from(tile_index, layout_index, dx, dy);
                        let operation_and_parent_index = Some((operation, own_index));
                        result.push(Node {
                            board: child_board,
                            operation_and_parent_index,
                        });
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug)]
pub struct State<'a> {
    /* Note that deduplication is non-trivial, since achieving the same silhouette does not
     * necessarily mean that the subtree will look exactly the same, because a different subset
     * of tiles might be remaining. Therefore, we don't even try.
     *
     * Invariants:
     * - `tiles.len() > 0`
     * - `closed` does not contain any solutions
     * - Depth-First-Search, to reduce memory strain.
     */
    closed: Vec<Node>,
    open: Vec<Node>,
    tiles: &'a [Tile],
}

pub type Result = Vec<Operation>;

impl<'a> State<'a> {
    pub fn new(initial_board: Board, tiles: &'a [Tile]) -> Self {
        Self {
            closed: vec![],
            open: vec![Node::new_root(initial_board)],
            tiles,
        }
    }

    pub fn can_step(&self) -> bool {
        !self.open.is_empty()
    }

    fn compute_remaining_tiles(&self, node: &Node) -> Vec<u8> {
        let mut bitvec = vec![true; self.tiles.len()];
        let mut walk_node = node;
        while let Some((operation, parent_index)) = &walk_node.operation_and_parent_index {
            let tile_index = usize::from(operation.indexed_tile_layout.tile_index);
            debug_assert!(bitvec[tile_index]);
            bitvec[tile_index] = false;
            walk_node = &self.closed[*parent_index];
        }
        bitvec
            .iter()
            .enumerate()
            .filter_map(|(tile_index, &tile_available)| {
                if tile_available {
                    Some(tile_index as u8)
                } else {
                    None
                }
            })
            .collect()
    }

    fn as_result(&self, node: &Node) -> Result {
        let mut result = Vec::with_capacity(self.tiles.len());
        let mut walk_node = node;
        while let Some((operation, parent_index)) = &walk_node.operation_and_parent_index {
            result.push(operation.clone());
            walk_node = &self.closed[*parent_index];
        }
        assert!(result.len() == self.tiles.len());
        result
    }

    pub fn step_single(&mut self) -> Option<Result> {
        let node = self.open.pop().expect("Forgot can_step()???");
        // TODO: Perhaps it is possible to drop all entries in `closed` beyond `node.parent_index`?
        let remaining_tile_indices = self.compute_remaining_tiles(&node);
        if remaining_tile_indices.len() == 0 {
            return Some(self.as_result(&node));
        }
        let next_parent_index = self.closed.len();

        // Now search for the tile which has the fewest places it can possibly go:
        let best_case_distinction = remaining_tile_indices
            .iter()
            .map(|&tile_index| {
                node.find_all_fits(
                    next_parent_index,
                    &self.tiles[tile_index as usize],
                    tile_index,
                )
            })
            .min_by_key(|case_distinction| case_distinction.len())
            .expect("List of available tiles is suddenly empty?!");
        // TODO: Short-circuit when a 0 has been found?

        if best_case_distinction.len() == 0 {
            // There is a tile which cannot be placed, therefore we don't need to consider this subtree at all.
            return None;
        }

        self.closed.push(node); // Only now 'next_parent_index' becomes actually valid!

        self.open.extend(best_case_distinction);

        // If any of the children are a solution, then *all* children are solutions,
        // so it's okay to delay by one step.
        None
    }

    pub fn step_at_most(&mut self, max_steps: usize) -> (usize, Option<Result>) {
        for steps_done in 0..max_steps {
            if !self.can_step() {
                return (steps_done, None);
            }
            let result_maybe = self.step_single();
            if result_maybe.is_some() {
                return (steps_done, result_maybe);
            }
        }

        (max_steps, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile;

    #[test]
    fn test_basic_negative() {
        let tiles: Vec<_> = vec![Tile::new_for_test(vec![0x0001])];
        let mut s = State::new(Board::all_blocked(), &tiles);
        assert_eq!(s.closed.len(), 0);
        assert_eq!(s.open.len(), 1);
        assert!(s.can_step());
        assert_eq!(s.step_single(), None);
        assert!(!s.can_step());
    }

    #[test]
    fn test_basic_positive() {
        let tiles: Vec<_> = vec![Tile::new_for_test(vec![0xFFFF, 0x0001])];
        let mut board = Board::all_blocked();
        board.set_unblocked(2, 3);
        let mut s = State::new(board, &tiles);
        assert_eq!(s.closed.len(), 0);
        assert_eq!(s.open.len(), 1);
        assert!(s.can_step());
        assert_eq!(s.step_single(), None);
        assert_eq!(s.step_single(), Some(vec![Operation::from(0, 1, 2, 3)]));
        assert!(!s.can_step());
    }

    #[test]
    fn test_trivial_positive() {
        let tiles: Vec<_> = vec![Tile::new_for_test(vec![0x0000])];
        let mut s = State::new(Board::all_blocked(), &tiles);
        assert_eq!(s.closed.len(), 0);
        assert_eq!(s.open.len(), 1);
        assert!(s.can_step());
        assert_eq!(s.step_single(), None);
        assert_eq!(
            s.step_single(),
            Some(vec![Operation::from(
                0,
                0,
                board::MAX_WIDTH - 1,
                board::MAX_HEIGHT - 1
            )])
        );
        // Will generate many more solutions, one for each possible offset.
        assert!(s.can_step());
    }

    #[test]
    fn test_notiles_positive() {
        let tiles: Vec<_> = vec![];
        let mut s = State::new(Board::all_blocked(), &tiles);
        assert_eq!(s.closed.len(), 0);
        assert_eq!(s.open.len(), 1);
        assert!(s.can_step());
        assert_eq!(s.step_single(), Some(vec![]));
        assert!(!s.can_step());
    }

    #[test]
    fn test_basic_positive_multi() {
        let tiles: Vec<_> = vec![Tile::new_for_test(vec![0xFFFF, 0x0001])];
        let mut board = Board::all_blocked();
        board.set_unblocked(2, 3);
        let mut s = State::new(board, &tiles);
        assert_eq!(
            s.step_at_most(5),
            (1, Some(vec![Operation::from(0, 1, 2, 3)]))
        );
        assert!(!s.can_step());
        assert_eq!(s.step_at_most(5), (0, None));
        assert!(!s.can_step());
        // Verify that we do not loop:
        assert_eq!(s.step_at_most(1234567890), (0, None));
    }

    #[test]
    fn test_twotile_positive_multi() {
        let tiles: Vec<_> = vec![
            Tile::new_for_test(vec![0x0011]),
            Tile::new_for_test(vec![0x0311]),
        ];
        let mut board = Board::all_blocked();
        board.set_unblocked(3, 0);
        board.set_unblocked(3, 1);
        board.set_unblocked(3, 2);
        board.set_unblocked(3, 3);
        board.set_unblocked(3, 4);
        board.set_unblocked(4, 4);
        let mut s = State::new(board, &tiles);
        assert_eq!(
            s.step_at_most(5),
            (
                2,
                Some(vec![
                    Operation::from(0, 0, 3, 0),
                    Operation::from(1, 0, 3, 2),
                ])
            )
        );
        assert_eq!(s.step_at_most(5), (0, None));
        assert!(!s.can_step());
        // Verify that we do not loop:
        assert_eq!(s.step_at_most(1234567890), (0, None));
    }

    #[test]
    fn test_sample_easy() {
        let tiles: Vec<_> = vec![
            tile::ALL_TILES[5].clone(),  // three-way pipe
            tile::ALL_TILES[6].clone(),  // S shape
            tile::ALL_TILES[10].clone(), // elongated three-way pipe
        ];
        let mut board = Board::all_blocked();
        board.set_unblocked(1, 0);
        board.set_unblocked(2, 0);
        board.set_unblocked(0, 1);
        board.set_unblocked(1, 1);
        board.set_unblocked(2, 1);
        board.set_unblocked(3, 1);
        board.set_unblocked(1, 2);
        board.set_unblocked(2, 2);
        board.set_unblocked(3, 2);
        board.set_unblocked(4, 2);
        board.set_unblocked(1, 3);
        board.set_unblocked(2, 3);
        board.set_unblocked(3, 3);
        let mut s = State::new(board, &tiles);
        assert_eq!(
            s.step_at_most(1000),
            (
                22,
                Some(vec![
                    Operation::from(0, 1, 2, 0),
                    Operation::from(1, 1, 2, 2),
                    Operation::from(2, 4, 0, 0),
                    // Visually:
                    // ·20··
                    // 2200·
                    // ·2011
                    // ·211·
                    // Good!
                ])
            )
        );
        // There is more than one solution
        assert!(s.can_step());
    }

    #[test]
    fn test_sample_easy_negative() {
        let tiles: Vec<_> = vec![
            tile::ALL_TILES[5].clone(),  // three-way pipe
            tile::ALL_TILES[6].clone(),  // S shape
            tile::ALL_TILES[10].clone(), // elongated three-way pipe
        ];
        let mut board = Board::all_blocked();
        board.set_unblocked(1, 0);
        board.set_unblocked(2, 0);
        board.set_unblocked(0, 1);
        board.set_unblocked(1, 1);
        board.set_unblocked(2, 1);
        board.set_unblocked(3, 1);
        board.set_unblocked(1, 2);
        board.set_unblocked(2, 2);
        board.set_unblocked(3, 2);
        board.set_unblocked(4, 2);
        board.set_unblocked(1, 3);
        board.set_unblocked(2, 3);
        // (3,3) missing, and (0,0) doesn't help.
        board.set_unblocked(0, 0);
        let mut s = State::new(board, &tiles);
        assert_eq!(s.step_at_most(1000), (29, None));
        assert!(!s.can_step());
    }
}
