//! sudoku solver application

#![warn(missing_docs)]

mod element;
mod error;
mod puzzle;

pub mod prelude {
    //! Common things to include in all modules

    pub use crate::element::Element;
    pub use crate::error::SudokuError;
    pub use crate::puzzle::Puzzle;
}
