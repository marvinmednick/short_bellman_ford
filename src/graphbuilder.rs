pub trait GraphBuilder {
    fn add_edge(&mut self, source: usize,dest: usize,weight: i64) -> Option<usize>;
    fn add_vertex(&mut self, id:  usize); 
}

