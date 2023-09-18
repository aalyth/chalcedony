
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Position {
    pub ln:  usize,
    pub col: usize,
}

impl Position {
    pub fn new(ln: usize, col: usize) -> Self {
        Position {
            ln,
            col,
        }
    }

    pub fn advance_col(&mut self) {
        self.col += 1;
    }

    pub fn advance_ln(&mut self) {
        self.ln += 1;
        self.col = 1;
    }
}
