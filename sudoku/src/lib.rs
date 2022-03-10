//! sudoku solver application

#![warn(missing_docs)]

mod element;
mod puzzle;

pub mod prelude {
    //! Common things to include in all modules

    pub use crate::element::Element;
    pub use crate::puzzle::Puzzle;
}
