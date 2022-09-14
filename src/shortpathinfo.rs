use crate::bellman::{MinMax};

#[derive(Debug)]
pub struct ShortestPathInfo {
    pub source: usize,
    pub dest: usize,
    pub distance: MinMax<i64>,
    pub path: Vec<usize>,
    pub has_negative_cycle: bool,
}
