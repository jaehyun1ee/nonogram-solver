#[derive(Debug, Clone)]
pub enum Cell {
    TRUE,
    FALSE,
    UNKNOWN,
}

impl Cell {
    pub fn print(&self) {
        match &self {
            Self::TRUE => print!("O"),
            Self::FALSE => print!("X"),
            Self::UNKNOWN => print!("?"),
        }
    }
}
