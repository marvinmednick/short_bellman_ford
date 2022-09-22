use crate::minmax::{MinMax};

#[derive(Debug,Clone)]
pub struct ShortestPathInfo {
    pub source: usize,
    pub dest: usize,
    pub distance: MinMax<i64>,
    pub path: Vec<usize>,
    pub path_len: usize,
    pub has_negative_cycle: bool,
}
