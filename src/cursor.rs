impl Cursor {
    pub fn merge(&mut self, other: Self) {
        self.pointer += other.pointer;
        if other.loc.line > 0 {
            self.loc.column = other.loc.column;
        } else {
            self.loc.column += other.loc.column;
        }
        self.loc.line += other.loc.line;
    }
}

#[derive(Default, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Default)]
pub struct Cursor {
    pub pointer: usize,
    pub loc: Location,
}

