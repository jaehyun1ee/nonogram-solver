mod board;
mod cell;
mod examples;
extern crate z3;

fn main() {
    let mut board = examples::easy_board();
    board.print();
    board.solve();
    board.print();

    let mut board = examples::hard_board();
    board.print();
    board.solve();
    board.print();

    let mut board = examples::extreme_board();
    board.print();
    board.solve();
    board.print();
}
