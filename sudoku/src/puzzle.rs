//! sudoku puzzle element

use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use anyhow::{anyhow, Context};
use bit_set::BitSet;
use tracing::{debug, info};

use crate::element::{Element, GROUP_SIZE};

const NUM_ELEMENTS: usize = GROUP_SIZE * GROUP_SIZE;

#[derive(Debug, Clone)]
enum PuzzleState {
    Solved,
    Unsolved,
    Unsolvable,
}

/// Represents a sudoku puzzle
#[derive(Clone)]
pub struct Puzzle {
    elements: [RefCell<Element>; NUM_ELEMENTS],
    state: PuzzleState,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            elements: [(); NUM_ELEMENTS].map(|_| RefCell::new(Element::default())),
            state: PuzzleState::Unsolved,
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.printer(f, false)
    }
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.printer(f, true)
    }
}

// Map from square number to array of element coordinates (row, col)
// for that square.  Note: The (row, col) is 1-based.
const SQR_TO_ROW_COL_MAP: [[(usize, usize); 9]; 9] = [
    [
        // square 1
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 1),
        (2, 2),
        (2, 3),
        (3, 1),
        (3, 2),
        (3, 3),
    ],
    // square 2
    [
        (1, 4),
        (1, 5),
        (1, 6),
        (2, 4),
        (2, 5),
        (2, 6),
        (3, 4),
        (3, 5),
        (3, 6),
    ],
    // square 3
    [
        (1, 7),
        (1, 8),
        (1, 9),
        (2, 7),
        (2, 8),
        (2, 9),
        (3, 7),
        (3, 8),
        (3, 9),
    ],
    [
        // square 4
        (4, 1),
        (4, 2),
        (4, 3),
        (5, 1),
        (5, 2),
        (5, 3),
        (6, 1),
        (6, 2),
        (6, 3),
    ],
    // square 5
    [
        (4, 4),
        (4, 5),
        (4, 6),
        (5, 4),
        (5, 5),
        (5, 6),
        (6, 4),
        (6, 5),
        (6, 6),
    ],
    // square 6
    [
        (4, 7),
        (4, 8),
        (4, 9),
        (5, 7),
        (5, 8),
        (5, 9),
        (6, 7),
        (6, 8),
        (6, 9),
    ],
    [
        // square 7
        (7, 1),
        (7, 2),
        (7, 3),
        (8, 1),
        (8, 2),
        (8, 3),
        (9, 1),
        (9, 2),
        (9, 3),
    ],
    // square 8
    [
        (7, 4),
        (7, 5),
        (7, 6),
        (8, 4),
        (8, 5),
        (8, 6),
        (9, 4),
        (9, 5),
        (9, 6),
    ],
    // square 9
    [
        (7, 7),
        (7, 8),
        (7, 9),
        (8, 7),
        (8, 8),
        (8, 9),
        (9, 7),
        (9, 8),
        (9, 9),
    ],
];

// Helper to map square number (1-9), into slice of (row, col)
fn map_sqr_to_row_col(sqr: usize) -> [(usize, usize); GROUP_SIZE] {
    let mut result = [(0, 0); GROUP_SIZE];

    for (i, (row, col)) in SQR_TO_ROW_COL_MAP[sqr].into_iter().enumerate() {
        result[i] = (row - 1, col - 1)
    }

    result
}

// Map a (row, cow) to a square
fn map_row_col_to_sqr(row: usize, col: usize) -> usize {
    let mut sqr = (row / 3) * 3;

    sqr += col / 3;
    sqr
}

// Map a (row, cow) to an offset within square
fn map_row_col_to_sqr_index(row: usize, col: usize) -> usize {
    let mut sqr_index = (row % 3) * 3;
    sqr_index += col % 3;
    sqr_index
}

impl Puzzle {
    /// Create a new puzzle from a file
    pub fn new(path: &Path) -> Result<Self, anyhow::Error> {
        let file = File::open(path)
            .with_context(|| format!("Failed to open puzzle file: {}", path.display()))?;

        let reader = BufReader::new(file);
        let mut puzzle_lines = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let line = line.with_context(|| {
                format!(
                    "Failed to read line puzzle file: {}:{}",
                    path.display(),
                    index + 1
                )
            })?;
            puzzle_lines.push(line);
        }

        let puzzle = Self::parse_puzzle(puzzle_lines)
            .with_context(|| format!("Failed to parse puzzle file: {}", path.display()))?;
        Ok(puzzle)
    }

    fn parse_puzzle(puzzle_lines: Vec<String>) -> Result<Self, anyhow::Error> {
        let mut puzzle = Puzzle::default();
        let mut row = 0;

        for (index, line) in puzzle_lines.iter().enumerate() {
            // parse the line -- [0-9X] separated by white space
            if line.is_empty() {
                continue;
            }

            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() != GROUP_SIZE {
                return Err(anyhow!(format!(
                    "Unexpected number of columns ({}) in puzzle line:{}",
                    fields.len(),
                    index + 1
                )));
            }

            for (col, field) in fields.into_iter().enumerate() {
                if field != "X" && field != "x" {
                    // try to parse as a number
                    let val: usize = field.parse().with_context(|| {
                        format!(
                            "Unable to parse element: {} as a number in puzzle line:col: {}:{}",
                            field,
                            index + 1,
                            col + 1
                        )
                    })?;
                    if val == 0 || val > GROUP_SIZE {
                        return Err(anyhow!(format!(
                            "Element value {} is out of range [1-9] in puzzle line:col: {}:{}",
                            val,
                            index + 1,
                            col + 1
                        )));
                    }
                    puzzle.finalize_element(row, col, val - 1);
                }
            }
            row += 1;
        }

        if row != GROUP_SIZE {
            return Err(anyhow!(format!("Not enough rows in puzzle: {}", row)));
        }

        Ok(puzzle)
    }

    // remove 'val' from every column of 'row'
    fn row_remove_possible(&mut self, row: usize, col: usize, val: usize) {
        for c in 0..GROUP_SIZE {
            if c != col {
                self.element_as_mut(row, c).remove(val);
            }
        }
    }

    // remove 'val' from every row of 'col'
    fn col_remove_possible(&mut self, row: usize, col: usize, val: usize) {
        for r in 0..GROUP_SIZE {
            if r != row {
                self.element_as_mut(r, col).remove(val);
            }
        }
    }

    // remove 'val' from every cell of 'sqr'
    fn sqr_remove_possible(&mut self, row: usize, col: usize, val: usize) {
        let sqr = map_row_col_to_sqr(row, col);
        for (r, c) in map_sqr_to_row_col(sqr) {
            if (r != row) && (c != col) {
                self.element_as_mut(r, c).remove(val);
            }
        }
    }

    fn finalize_element(&mut self, row: usize, col: usize, val: usize) {
        self.element_as_mut(row, col).finalize(val);

        // remove val as possible from other row, cel, square groups
        self.row_remove_possible(row, col, val);
        self.col_remove_possible(row, col, val);
        self.sqr_remove_possible(row, col, val);
    }

    fn element(&self, row: usize, col: usize) -> Ref<'_, Element> {
        self.elements[(row * GROUP_SIZE) + col].borrow()
    }

    fn element_as_mut(&self, row: usize, col: usize) -> RefMut<'_, Element> {
        self.elements[(row * GROUP_SIZE) + col].borrow_mut()
    }

    fn printer(&self, f: &mut std::fmt::Formatter<'_>, debug: bool) -> std::fmt::Result {
        writeln!(f, "Puzzle state: {:?}", self.state)?;
        for i in 0..GROUP_SIZE {
            for j in 0..GROUP_SIZE {
                if debug {
                    write!(f, "{:?} ", self.element(i, j))?;
                } else {
                    write!(f, "{} ", self.element(i, j))?;
                }
                if ((j + 1) % 3) == 0 {
                    write!(f, "  ")?;
                }
            }
            writeln!(f)?;
            if (i == 2) || (i == 5) {
                writeln!(f)?;
            }
        }
        write!(f, "")
    }

    fn is_complete(&self) -> bool {
        !self.elements.iter().any(|e| !e.borrow().is_finalized())
    }

    fn reduce_basic_elements(&mut self) -> usize {
        let mut updates = 0;

        // This is the most basic reduction. Finding elements that are
        // obviously done and finalizing them.
        //
        // Repeat until no more changes
        loop {
            let last_updates = updates;

            for r in 0..GROUP_SIZE {
                for c in 0..GROUP_SIZE {
                    let element = self.element(r, c);
                    if let Some(v) = element.ready() {
                        drop(element);
                        debug!("reduce_basic: row: {}, col: {}, must be: {}", r, c, v + 1);
                        self.finalize_element(r, c, v);
                        updates += 1;
                    }
                }
            }

            if last_updates == updates {
                // No changes this roundup
                break;
            }
        }

        updates
    }

    fn row_minus(&self, row: usize, minus_set: &BitSet) -> Vec<RefMut<'_, Element>> {
        let mut group = Vec::new();

        for c in 0..GROUP_SIZE {
            if minus_set.contains(c) {
                continue;
            }
            group.push(self.element_as_mut(row, c))
        }

        group
    }

    fn diff_other_group(
        &self,
        row: usize,
        col: usize,
        other: Vec<RefMut<'_, Element>>,
    ) -> Option<usize> {
        debug!("diff_other: r: {}, c {}, other:{:?}", row, col, other);
        // create union of the other elements
        let other_union = other.as_slice().iter().fold(BitSet::new(), |mut u, e| {
            debug!(
                "diff_other: Union: {:?} with possible: {:?}",
                u,
                &e.borrow().possible()
            );
            u.union_with(&e.borrow().possible());
            u
        });
        let diff: Vec<_> = self
            .element(row, col)
            .borrow()
            .possible()
            .difference(&other_union)
            .collect();

        debug!(
            "diff_other: this possible: {:?}, other_union: {:?}",
            self.element(row, col).borrow().possible(),
            other_union
        );
        debug!("diff_other: possible - other = {:?}", diff);

        if diff.len() == 1 {
            debug!("diff_other: solved: {}", diff[0]);
            Some(diff[0])
        } else {
            None
        }
    }

    fn row_scan(&mut self) -> usize {
        let mut updates = 0;

        // for each row
        //   for element in this row
        //     check if this_element *minus* all other elements contains a single possibility
        for r in 0..GROUP_SIZE {
            for c in 0..GROUP_SIZE {
                // skip if already finalized
                if self.element(r, c).borrow().is_finalized() {
                    continue;
                }

                // skip current col
                let mut this_set = BitSet::with_capacity(GROUP_SIZE);
                this_set.insert(c);
                let other_elements = self.row_minus(r, &this_set);

                if let Some(val) = self.diff_other_group(r, c, other_elements) {
                    // found one.  finalize this value.
                    info!("row_scan: found one, row: {}, col: {}, val: {}", r, c, val);
                    self.finalize_element(r, c, val);
                    updates += 1;
                }
            }
        }

        updates
    }

    fn col_minus(&self, col: usize, minus_set: &BitSet) -> Vec<RefMut<'_, Element>> {
        let mut group = Vec::new();

        for r in 0..GROUP_SIZE {
            if minus_set.contains(r) {
                continue;
            }
            group.push(self.element_as_mut(r, col))
        }

        group
    }

    fn col_scan(&mut self) -> usize {
        let mut updates = 0;

        // for each col
        //   for element in this col
        //     check if this_element *minus* all other elements contains a single possibility
        for c in 0..GROUP_SIZE {
            for r in 0..GROUP_SIZE {
                // skip if already finalized
                if self.element(r, c).borrow().is_finalized() {
                    continue;
                }

                // skip current row
                let mut this_set = BitSet::with_capacity(GROUP_SIZE);
                this_set.insert(r);

                let other_elements = self.col_minus(c, &this_set);

                if let Some(val) = self.diff_other_group(r, c, other_elements) {
                    // found one.  finalize this value.
                    info!("col_scan: found one, row: {}, col: {}, val: {}", r, c, val);
                    self.finalize_element(r, c, val);
                    updates += 1;
                }
            }
        }

        updates
    }

    fn sqr_minus(&self, row: usize, col: usize, minus_set: &BitSet) -> Vec<RefMut<'_, Element>> {
        let sqr = map_row_col_to_sqr(row, col);

        debug!(
            "sqr_minus: row: {}, col: {}, sqr: {}, minus_set: {:?}",
            row, col, sqr, minus_set
        );
        let mut group = Vec::new();
        for (r, c) in map_sqr_to_row_col(sqr) {
            if minus_set.contains(map_row_col_to_sqr_index(r, c)) {
                continue;
            }
            debug!("  sqr_minus: pushing (r, c): {}, {}", r, c);
            group.push(self.element_as_mut(r, c));
        }

        group
    }

    fn sqr_scan(&mut self) -> usize {
        let mut updates = 0;

        // loop over squares
        for sqr in 0..GROUP_SIZE {
            for (r, c) in map_sqr_to_row_col(sqr) {
                // skip if already finalized
                if self.element(r, c).borrow().is_finalized() {
                    continue;
                }

                // skip current square index
                let mut this_set = BitSet::with_capacity(GROUP_SIZE);
                this_set.insert(map_row_col_to_sqr_index(r, c));

                let other_elements = self.sqr_minus(r, c, &this_set);

                if let Some(val) = self.diff_other_group(r, c, other_elements) {
                    // found one.  finalize this value.
                    info!("sqr_scan: found one, row: {}, col: {}, val: {}", r, c, val);
                    self.finalize_element(r, c, val);
                    updates += 1;
                }
            }
        }

        updates
    }

    fn reduce(&mut self) {
        debug!("Before reduce_basic_elements(): {:?}", &self);
        let mut updates = self.reduce_basic_elements();

        debug!("Before row_scan(): updates: {}, {:?}", updates, &self);
        updates += self.row_scan();
        debug!("Before col_scan(): updates: {}, {:?}", updates, &self);
        updates += self.col_scan();
        debug!("Before sqr_scan(): updates: {}, {:?}", updates, &self);
        updates += self.sqr_scan();

        // TODO: medium hidden pairs scan

        if updates == 0 {
            // Time to stop
            self.state = PuzzleState::Unsolvable;
        } else if self.is_complete() {
            // Time to stop
            self.state = PuzzleState::Solved;
        } else {
            // Keep going
            self.state = PuzzleState::Unsolved;
        }
    }

    /// Attempt to solve the puzzle
    pub fn solve(&mut self) -> Result<usize, anyhow::Error> {
        let mut i = 0;
        // loop while !puzzle.solved()
        // states:
        // - solved
        // - unsolved
        // - unsolvable
        info!("Starting to solve puzzle");
        loop {
            debug!("Iter: {}\n{}", i, &self);

            match self.state {
                PuzzleState::Unsolved => self.reduce(),
                PuzzleState::Solved => break,
                PuzzleState::Unsolvable => break,
            }

            i += 1;
        }

        match self.state {
            PuzzleState::Solved => Ok(i),
            _ => Err(anyhow!(format!(
                "Puzzle failed: {:?}, iterations: {}",
                self.state, i
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_data(data: Vec<&str>) -> Result<Puzzle, anyhow::Error> {
        let data_string = data.iter().map(|s| s.to_string()).collect();
        Puzzle::parse_puzzle(data_string)
    }

    #[test]
    fn row_col_to_sqr_index_map() {
        for x in 0..3 {
            for y in 0..3 {
                for r in 0..3 {
                    for c in 0..3 {
                        assert_eq!(
                            map_row_col_to_sqr_index(r + (x * 3), c + (y * 3)),
                            (r * 3) + c
                        )
                    }
                }
            }
        }
    }

    #[test]
    fn parse_good_puzzle() {
        let good_num_cols = vec![
            "8 7 X 1 X X X X X",
            "X X 2 X X X 1 X 4",
            "X X X X 5 9 7 8 X",
            "",
            "3 X X 4 X 6 X X X",
            "X X 7 X X X 9 X X",
            "X X X 8 X 3 X X 6",
            "",
            "X 4 5 9 X X X X X",
            "2 X 3 X X X 4 X X",
            "X X X X X 7 X 5 9",
        ];

        let result = parse_data(good_num_cols);
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_bad_puzzle() {
        let bad_num_cols = vec![
            "8 7 X 1 X X X X",
            "X X 2 X X X 1 X 4",
            "X X X X 5 9 7 8 X",
            "",
            "3 X X 4 X 6 X X X",
            "X X 7 X X X 9 X X",
            "X X X 8 X 3 X X 6",
            "",
            "X 4 5 9 X X X X X",
            "2 X 3 X X X 4 X X",
            "X X X X X 7 X 5 9",
        ];
        let result = parse_data(bad_num_cols);
        println!("result: {:?}", result);
        assert!(result.is_err());

        let bad_num_rows = vec![
            "8 7 X 1 X X X X X",
            "X X 2 X X X 1 X 4",
            "X X X X 5 9 7 8 X",
        ];
        let result = parse_data(bad_num_rows);
        println!("result: {:?}", result);
        assert!(result.is_err());

        let bad_not_valid_char = vec![
            "8 7 X 1 X X X X X",
            "X X 2 X X X 1 X 4",
            "X X X X 5 9 7 8 X",
            "",
            "3 X X 4 X 6 X X X",
            "X X 7 X X X 9 X X",
            "X X X 8 * 3 X X 6",
            "",
            "X 4 5 9 X X X X X",
            "2 X 3 X X X 4 X X",
            "X X X X X 7 X 5 9",
        ];

        let result = parse_data(bad_not_valid_char);
        println!("result: {:?}", result);
        assert!(result.is_err());

        let bad_not_valid_number1 = vec![
            "8 7 X 1 X X X X X",
            "X X 2 X X X 1 X 4",
            "X X X X 5 9 7 8 X",
            "",
            "3 X X 4 X 6 X X X",
            "X X 7 X 0 X 9 X X",
            "X X X 8 X 3 X X 6",
            "",
            "X 4 5 9 X X X X X",
            "2 X 3 X X X 4 X X",
            "X X X X X 7 X 5 9",
        ];

        let result = parse_data(bad_not_valid_number1);
        println!("result: {:?}", result);
        assert!(result.is_err());

        let bad_not_valid_number2 = vec![
            "8 7 X 1 X X X X X",
            "X X 2 X X X 1 X 4",
            "X X X X 5 9 7 8 X",
            "",
            "3 X X 4 X 6 X X X",
            "X X 7 X 10 X 9 X X",
            "X X X 8 X 3 X X 6",
            "",
            "X 4 5 9 X X X X X",
            "2 X 3 X X X 4 X X",
            "X X X X X 7 X 5 9",
        ];

        let result = parse_data(bad_not_valid_number2);
        println!("result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn solve_good_puzzles() {
        let input1 = vec![
            "8 X X X 4 6 2 9 X",
            "7 X X X X 9 X X 5",
            "X X 2 X X 5 X X X",
            "",
            "X 6 X 2 1 X 8 4 X",
            "X 2 7 X 8 X 5 3 X",
            "X 3 8 X 6 7 X 2 X",
            "",
            "X X X 4 X X 6 X X",
            "9 X X 3 X X X X X",
            "X 4 1 6 5 X X X 3",
        ];
        let mut puzzle = parse_data(input1).unwrap();
        assert!(puzzle.solve().is_ok());

        let input2 = vec![
            "5 4 X 6 X X X 2 X",
            "1 X 8 X 3 X X X 4",
            "X X 2 5 X X X 7 X",
            "",
            "X X X X X 7 2 5 X",
            "X X 5 8 X 9 4 X X",
            "X 2 6 3 X X X X X",
            "",
            "X 6 X X X 3 7 X X",
            "4 X X X 9 X 6 X 2",
            "X 8 X X X 6 X 4 5",
        ];
        let mut puzzle = parse_data(input2).unwrap();
        assert!(puzzle.solve().is_ok());

        let input3 = vec![
            "8 1 X X X X X X X",
            "X X 9 X X 4 2 8 X",
            "X X X X X 1 6 X 9",
            "5 7 X X X 9 X X 8",
            "X X X X 7 X X X X",
            "9 X X 1 X X X 5 4",
            "3 X 1 2 X X X X X",
            "X 4 6 3 X X 8 X X",
            "X X X X X X X 3 6",
        ];
        let mut puzzle = parse_data(input3).unwrap();
        assert!(puzzle.solve().is_ok());

        let input4 = vec![
            "X 3 5 X X X 9 X X",
            "X X X X 1 6 X 5 X",
            "X 2 X 4 X X 8 X X",
            "X X X 5 X 2 X X 4",
            "X X 4 X 8 X 2 X X",
            "9 X X 6 X 4 X X X",
            "X X 9 X X 5 X 8 X",
            "X 1 X 8 6 X X X X",
            "X X 6 X X X 7 2 X",
        ];
        let mut puzzle = parse_data(input4).unwrap();
        assert!(puzzle.solve().is_ok());

        let input5 = vec![
            "X X X X 4 X X X 3",
            "X X 2 X X X X 9 7",
            "X 6 X X X 3 1 2 X",
            "6 X X 8 X 9 X X 1",
            "X X 9 X X X 2 X X",
            "2 X X 3 X 6 X X 5",
            "X 1 4 6 X X X 5 X",
            "5 9 X X X X 7 X X",
            "7 X X X 1 X X X X",
        ];
        let mut puzzle = parse_data(input5).unwrap();
        assert!(puzzle.solve().is_ok());
    }
}
