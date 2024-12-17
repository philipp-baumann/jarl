use serde::{Deserialize, Serialize};

/// Sourcecode location.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Location {
    pub(super) row: usize,
    pub(super) column: usize,
}

impl Location {
    pub fn fmt_with(
        &self,
        f: &mut std::fmt::Formatter,
        e: &impl std::fmt::Display,
    ) -> std::fmt::Result {
        write!(f, "{} at line {} column {}", e, self.row(), self.column())
    }
}

impl Location {
    /// Creates a new Location object at the given row and column.
    ///
    /// # Example
    /// ```
    /// use rustpython_compiler_core::Location;
    /// let loc = Location::new(10, 10);
    /// ```
    pub fn new(row: usize, column: usize) -> Self {
        Location { row, column }
    }

    /// Current row
    pub fn row(&self) -> usize {
        self.row
    }

    /// Current column
    pub fn column(&self) -> usize {
        self.column
    }

    pub fn reset(&mut self) {
        self.row = 1;
        self.column = 1;
    }

    pub fn go_right(&mut self) {
        self.column += 1;
    }

    pub fn go_left(&mut self) {
        self.column -= 1;
    }

    pub fn newline(&mut self) {
        self.row += 1;
        self.column = 1;
    }
}
