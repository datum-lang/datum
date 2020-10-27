#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Loc(pub usize, pub usize, pub usize);

impl Loc {
    pub fn new(file_no: usize, start: usize, end: usize) -> Self {
        Loc(file_no, start, end)
    }
}
