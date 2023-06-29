mod board;
mod cell;
mod examples;
mod solver;

use solver::NonogramSolver;

extern crate z3;

fn main() {
    let board = examples::easy_board();
    board.print();
    let z3_nonogram_solver = solver::sat::Z3Solver::new(board);
    z3_nonogram_solver.solve().expect("solver failed").print();

    let board = examples::hard_board();
    board.print();
    let z3_nonogram_solver = solver::sat::Z3Solver::new(board);
    z3_nonogram_solver.solve().expect("solver failed").print();

    let board = examples::extreme_board();
    board.print();
    let z3_nonogram_solver = solver::sat::Z3Solver::new(board);
    z3_nonogram_solver.solve().expect("solver failed").print();
}
