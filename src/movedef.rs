#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Movedef {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) taken_piece: Option<usize>,
}