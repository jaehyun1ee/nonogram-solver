use crate::cell::Cell;

#[derive(Debug, Clone)]
pub struct Board {
    pub board: Vec<Vec<Cell>>,
    pub height: usize,
    pub width: usize,
    pub rows: Vec<Vec<usize>>,
    pub cols: Vec<Vec<usize>>,
}

impl Board {
    pub fn new(rows: Vec<Vec<usize>>, cols: Vec<Vec<usize>>, hints: Vec<(usize, usize)>) -> Self {
        let height = rows.len();
        let width = cols.len();

        let mut board = Vec::new();
        for i in 0..height {
            let mut row = Vec::new();
            for j in 0..width {
                let cell = if hints.contains(&(i, j)) {
                    Cell::FALSE
                } else {
                    Cell::UNKNOWN
                };
                row.push(cell);
            }
            board.push(row);
        }

        Self {
            board,
            height,
            width,
            rows,
            cols,
        }
    }

    pub fn print(&self) {
        let cons = &self.rows.iter().fold(
            0,
            |cons, row| if cons < row.len() { row.len() } else { cons },
        );
        let &cons = &self.cols.iter().fold(
            *cons,
            |cons, col| if cons < col.len() { col.len() } else { cons },
        );

        for i in 0..cons + self.height {
            for j in 0..cons + self.width {
                // Print placeholder.
                if i < cons && j < cons {
                    print!("{:^2}", "#");
                }
                // Print row constraints.
                else if i >= cons && j < cons {
                    let line = &self.rows[i - cons];

                    let idx = cons - j;
                    if idx <= line.len() {
                        print!("{:^2}", line[line.len() - idx]);
                    } else {
                        print!("{:^2}", " ");
                    }
                }
                // Print column constraints.
                else if i < cons && j >= cons {
                    let line = &self.cols[j - cons];

                    let idx = cons - i;
                    if idx <= line.len() {
                        print!("{:^2}", line[line.len() - idx]);
                    } else {
                        print!("{:^2}", " ");
                    }
                }
                // Print board.
                else {
                    let cell = &self.board[i - cons][j - cons];
                    cell.print();
                }

                print!("{:^2}", " ");
            }
            println!();
        }
    }
}
