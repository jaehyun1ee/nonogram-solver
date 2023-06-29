use crate::board::Board;

pub mod sat;

pub trait NonogramSolver {
    fn solve(&self) -> Option<Board>;
}
