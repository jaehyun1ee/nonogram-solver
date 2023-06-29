use z3::{ast::Bool, Config, Context, Model, SatResult, Solver};

use crate::{board::Board, cell::Cell};

use super::NonogramSolver;

pub struct Z3Solver {
    pub board: Board,
}

impl NonogramSolver for Z3Solver {
    fn solve(&self) -> Option<Board> {
        let mut config = Config::new();
        config.set_proof_generation(true);
        let context = Context::new(&config);

        let solver = Solver::new(&context);
        solver.push();

        let eboard = self.encode_board(&context);

        let ehints = self.encode_hints(&eboard, &context);
        solver.assert(&ehints);

        let erows = self.encode_rows(&eboard, &context);
        solver.assert(&erows);

        let ecols = self.encode_cols(&eboard, &context);
        solver.assert(&ecols);

        match solver.check() {
            SatResult::Sat => {
                println!("SAT");
                let sboard = solver.get_model().unwrap();
                Some(self.decode_board(&eboard, &sboard))
            }
            SatResult::Unsat => {
                println!("UNSAT");
                None
            }
            SatResult::Unknown => {
                println!("UNKNOWN");
                None
            }
        }
    }
}

impl Z3Solver {
    pub fn new(board: Board) -> Self {
        Self { board }
    }

    fn encode_board<'c>(&self, context: &'c Context) -> Vec<Vec<Bool<'c>>> {
        let mut eboard = Vec::new();
        for r in 0..self.board.height {
            let mut erow = Vec::new();
            for c in 0..self.board.width {
                let ecell = Bool::new_const(&context, format!("{},{}", r, c));
                erow.push(ecell);
            }
            eboard.push(erow);
        }

        eboard
    }

    fn transpose_board<'c>(eboard: &Vec<Vec<Bool<'c>>>) -> Vec<Vec<Bool<'c>>> {
        let height = eboard.len();
        let width = eboard[0].len();

        let mut eboard_t = Vec::new();
        for c in 0..width {
            let mut erow_t = Vec::new();
            for r in 0..height {
                let ecell = eboard[r][c].clone();
                erow_t.push(ecell);
            }
            eboard_t.push(erow_t);
        }

        eboard_t
    }

    fn decode_board<'c>(&self, eboard: &Vec<Vec<Bool<'c>>>, sboard: &Model<'c>) -> Board {
        let mut board = self.board.clone();
        for r in 0..self.board.height {
            for c in 0..self.board.width {
                let ecell = &eboard[r][c];
                let scell = Bool::as_bool(&sboard.eval(ecell, true).unwrap()).unwrap();

                board.board[r][c] = if scell { Cell::TRUE } else { Cell::FALSE };
            }
        }
        board
    }

    fn encode_hints<'c>(&self, eboard: &Vec<Vec<Bool<'c>>>, context: &'c Context) -> Bool<'c> {
        let mut ehints = Vec::new();
        for r in 0..self.board.height {
            for c in 0..self.board.width {
                let ecell = &eboard[r][c];

                match &self.board.board[r][c] {
                    Cell::TRUE => ehints.push(ecell.clone()),
                    Cell::FALSE => ehints.push(ecell.not()),
                    _ => (),
                };
            }
        }

        let ehints: Vec<&Bool<'c>> = ehints.iter().collect();
        let ehints = Bool::and(&context, ehints.as_slice());

        ehints
    }

    fn encode_rows<'c>(&self, eboard: &Vec<Vec<Bool<'c>>>, context: &'c Context) -> Bool<'c> {
        Self::encode_lines(&self.board.rows, &eboard, &context)
    }

    fn encode_cols<'c>(&self, eboard: &Vec<Vec<Bool<'c>>>, context: &'c Context) -> Bool<'c> {
        Self::encode_lines(&self.board.cols, &Self::transpose_board(eboard), &context)
    }

    fn encode_lines<'c>(
        lines: &Vec<Vec<usize>>,
        eboard: &Vec<Vec<Bool<'c>>>,
        context: &'c Context,
    ) -> Bool<'c> {
        let height = eboard.len();
        let width = eboard[0].len();

        let mut elines = Vec::new();
        for r in 0..height {
            let line = &lines[r];

            let mut eline = Vec::new();
            let splits = split_line(&line, width);
            for split in splits {
                let mut c = 0;

                let mut esplit = Vec::new();
                for i in 0..split.len() {
                    // Uncolored cells.
                    let space = split[i];
                    let space = if i == 0 || i == split.len() - 1 {
                        space
                    } else {
                        space + 1
                    };
                    for _ in 0..space {
                        let ecell = &eboard[r][c];
                        esplit.push(ecell.not());
                        c += 1;
                    }

                    if i == split.len() - 1 {
                        break;
                    }

                    // Colored cells.
                    let space = line[i];
                    for _ in 0..space {
                        let ecell = &eboard[r][c];
                        esplit.push(ecell.clone());
                        c += 1;
                    }
                }

                let esplit: Vec<&Bool<'c>> = esplit.iter().collect();
                let esplit = Bool::and(&context, esplit.as_slice());
                eline.push(esplit);
            }

            let eline: Vec<&Bool<'_>> = eline.iter().collect();
            let eline = Bool::or(&context, eline.as_slice());
            elines.push(eline);
        }

        let elines: Vec<&Bool<'_>> = elines.iter().collect();
        let elines = Bool::and(&context, elines.as_slice());

        elines
    }
}

fn split_line(line: &Vec<usize>, length: usize) -> Vec<Vec<usize>> {
    // Number of intervals between consecutive colored cells.
    let nintervals = line.len() - 1;
    // Number of chunks of consecutive uncolored cells.
    let nchunks = nintervals + 2;
    // Number of uncolored cells.
    let leftovers = length - (line.iter().fold(0, |x, y| x + y) + nintervals);

    // Split leftovers among spaces.
    let mut splits = Vec::new();
    let mut current = vec![0; nchunks];
    split(leftovers, nchunks, 0, &mut current, &mut splits);

    splits
}

fn split(
    n: usize,
    buckets: usize,
    index: usize,
    current: &mut Vec<usize>,
    splits: &mut Vec<Vec<usize>>,
) {
    if index == buckets - 1 {
        current[index] = n;
        splits.push(current.clone());
        return;
    }

    for i in 0..=n {
        current[index] = i;
        split(n - i, buckets, index + 1, current, splits);
    }
}
