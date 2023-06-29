#[derive(Debug, Clone)]
pub enum Cell {
    TRUE,
    FALSE,
    UNKNOWN,
}

impl Cell {
    pub fn print(&self) {
        match &self {
            Self::TRUE => print!("{:^2}", "O"),
            Self::FALSE => print!("{:^2}", "X"),
            Self::UNKNOWN => print!("{:^2}", "?"),
        }
    }
}
