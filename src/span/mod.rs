/// Represents a region in a source code, useful for error reporting.
pub struct Span {
    pub start_idx: usize,
    pub end_idx: usize,
    pub len: usize,
}

impl Span {
    pub fn new(start_idx: usize, end_idx: usize) -> Self {
        Span {
            start_idx,
            end_idx,
            len: end_idx - start_idx,
        }
    }
}
