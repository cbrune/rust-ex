//! sudoku puzzle element

use std::fmt;

use bit_set::BitSet;

pub const GROUP_SIZE: usize = 9;

/// Represents a cell in a Sudoku puzzle
#[derive(Clone)]
pub struct Element {
    resolved: Option<usize>,
    possible: BitSet,
}

impl Element {
    /// Initialize an element to a finalized value
    pub fn finalize(&mut self, val: usize) {
        self.resolved = Some(val);
        self.possible.clear();
        self.possible.insert(val);
    }

    /// Test if element is finalized
    pub fn is_finalized(&self) -> bool {
        self.resolved.is_some()
    }

    /// Remove a possibility
    pub fn remove(&mut self, val: usize) {
        self.possible.remove(val);
    }

    /// Return the possibility set
    pub fn possible(&self) -> BitSet {
        self.possible.clone()
    }

    /// Check whether element has only one bit, but not finalized
    pub fn ready(&self) -> Option<usize> {
        match self.resolved {
            Some(_) => None,
            None => {
                if self.possible.len() == 1 {
                    // Element is final
                    // iter with one val has deterministic order
                    let val = self.possible.iter().next().unwrap();
                    Some(val)
                } else {
                    None
                }
            }
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        let mut possible = BitSet::with_capacity(GROUP_SIZE);
        for i in 0..GROUP_SIZE {
            possible.insert(i);
        }
        Self {
            resolved: None,
            possible,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.resolved {
            Some(v) => {
                write!(f, "{}", v + 1)
            }
            None => {
                write!(f, "X")
            }
        }
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.resolved {
            Some(v) => {
                write!(f, "{}:{:?}", v + 1, self.possible.get_ref())
            }
            None => {
                write!(f, "X:{:?}", self.possible.get_ref())
            }
        }
    }
}
