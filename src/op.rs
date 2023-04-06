#[derive(Debug, Eq, PartialEq)]
pub enum Op {
    Add(usize, usize, usize),
    Minus(usize, usize),
    Mul(usize, usize, usize),
}
