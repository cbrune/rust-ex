//! Sample Error codes

use thiserror::Error;

/// Enum of error types
#[derive(Error, Debug)]
pub enum SudokuError {
    /// Inconsistent puzzle state detected
    #[error("Puzzle state inconsistent: row: {0}, col: {0}, val: {0}")]
    PuzzleStateInconsistent(usize, usize, usize),

    /// Puzzle unsolved, but forward progress made
    #[error("Puzzle unsolved")]
    PuzzleUnsolved,

    /// Puzzle unsolvable - no forward progress made
    #[error("Puzzle unsolvable")]
    PuzzleUnsolvable,
}
