extern crate two_d_array;
use std::collections::{BTreeMap};
//use two_d_array::TwoDArray;

use crate::dirgraph::DirectedGraph;
use crate::graphbuilder::GraphBuilder;

use log::{ info, error, debug /*,warn */,  trace };

// use std::fmt;
use crate::bellman::{Bellman,MinMax};
use crate::dijkstra::Dijkstra;
use crate::bellman::MinMax::{Value};

#[derive(Debug)]
pub struct Johnson<'a> {
        graph : &'a mut DirectedGraph,
        shortest_path_lengths:  BTreeMap::<usize,BTreeMap::<usize,MinMax<i64>>>,
        found_negative_cycle : bool,
        num_vertex: usize

}


impl<'a> Johnson<'a> {

    pub fn new(mut graph: &mut DirectedGraph ) -> Johnson {

        // Create a vertex with id 0 with a 0 cost edge to all vertex
        // to ensure there is  starting vertex that is connectted to 
        // all other vertex to for Bellman anaylsys
        if graph.define_vertex(0) == None {
            error!("Vertex 0 aready exists can't continue");
            panic!("Vertex 0 already exists")
        }
        else {
            // add the edges to create connections between 0 and all other vertexes
            for v in graph.get_vertex_ids() {
                graph.add_edge(0,v,0);
            }
        }


        let num_vertex = graph.vertex_count();

        Johnson {
            graph    : graph,
            shortest_path_lengths:  BTreeMap::<usize,BTreeMap::<usize,MinMax<i64>>>::new(),
            found_negative_cycle : false,
            num_vertex:  num_vertex,
        }

    }

    /// Find all shortest path from each vertex to all other vertes
    pub fn calculate_shortest_paths(&mut self) {
        let mut adjustment_info = Bellman::new(self.num_vertex);
        let mut g_prime  = DirectedGraph::new();

        info!("Starting all shortest path analysis with Johnson algorithm");
        self.found_negative_cycle = false;


        info!("Staring Bellman");
        adjustment_info.calculate_shortest_paths(self.graph, 0);
        let adjustment_results = adjustment_info.get_shortest_path_distances();
        self.found_negative_cycle = adjustment_info.has_negative_cycle();
        info!("Adjustment negative_cycle? {}",self.found_negative_cycle );
        info!("Adjustment results {:?} ", adjustment_results);


        if !self.found_negative_cycle {

            // create the graph with the adjusted edge weights cacluated as preveious edge +
            // source_vertex adjustment - dest_vertext adjustment
            // skip vertex 0 since we added that to ensure that there a connected graph from the
            // starting vertex 
            for (_id, edge) in self.graph.edge_iter() {
                if edge.source() != 0 {
                    if let (Value(source_adj), Value(dest_adj))  = (adjustment_results[&edge.source()], adjustment_results[&edge.dest()]) {
                        let adj_weight =  edge.weight() + source_adj - dest_adj;
                        (&mut g_prime).add_edge(edge.source(),edge.dest(),adj_weight);
                    }
                    else {
                        error!("Non numeric adjustment values source: {} dest {} source adj: {} dest adj {}",
                               edge.source(),
                               edge.dest(),
                               adjustment_results[&edge.source()],
                               adjustment_results[&edge.dest()]
                            );
                    }
                }
                

            }
            debug!("g_prime {:#?}",g_prime);


            for start in 1..self.num_vertex {
                let mut d = Dijkstra::new(start);

                for (id, _v) in g_prime.vertex_iter() {
                    d.initialize_vertex(id.clone());
                }
                d.calculate_shortest_paths(&g_prime, start);
                let mut results = d.get_shortest_path_distances();
                trace!("Results for Starting Vertex {} Before adjustment correction", start);
                trace!("{:?}",results);
                for (vertex_id, distance) in results.iter_mut() {
                    *distance = *distance - adjustment_results[&start] + adjustment_results[vertex_id];
                }
                info!("Results for Starting Vertex {} AFTER adjustment correction", start);
                info!("{:?}",results);
                self.shortest_path_lengths.insert(start,results);

            }

        }
        else {
            info!("Found Negative Cycle")
        }

    }

    pub fn has_negative_cycle(&self) -> bool {
        self.found_negative_cycle
    }
    
        
    pub fn results_iter(&self) -> std::collections::btree_map::Iter<'_, usize, BTreeMap::<usize,MinMax<i64>>>
    {
        self.shortest_path_lengths.iter()

    }

    pub fn shortest_shortest_path(&self) -> (MinMax<i64>, (MinMax<usize>,MinMax<usize>)){
        let mut min_path_len = MinMax::NA;
        let mut min_path = (MinMax::NA,MinMax::NA);
        for (start, path_lengths) in self.shortest_path_lengths.iter() {
            for (dest, len) in path_lengths.iter() {
                if *len < min_path_len {
                    min_path_len = *len;
                    min_path = (Value(*start),Value(*dest));
                }

            }

        }
        (min_path_len, min_path)
        
    }


}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use crate::dirgraph::DirectedGraph;
    use crate::graphbuilder::GraphBuilder;
    use crate::Johnson;
    use log::{  info , /*error, debug, warn, trace */ };

    fn init() {
      let _ = env_logger::builder().is_test(true).try_init();
      info!("Init {}",module_path!());
    }

	fn setup_basic(mut g :&mut DirectedGraph) { 
		assert_eq!(g.add_edge(1,2,12),Some(1));
		assert_eq!(g.add_edge(1,3,-13),Some(2));
		assert_eq!(g.add_edge(2,3,23),Some(3));
		assert_eq!(g.add_edge(2,4,-24),Some(4));
		assert_eq!(g.add_edge(3,4,34),Some(5));
		assert_eq!(g.add_edge(4,5,-45),Some(6));
		assert_eq!(g.get_outgoing_vertex_ids(1),&[2,3]);
		assert_eq!(g.get_outgoing_vertex_ids(2),&[3,4]);
		assert_eq!(g.get_outgoing_vertex_ids(3),&[4]);
		assert_eq!(g.get_outgoing_vertex_ids(4),&[5]);
	} 

    #[test]
    fn basic() {
        init();
		let mut g = DirectedGraph::new();
        setup_basic(&mut g); 
        info!("basic Setup complete");
        let mut j = Johnson::<'_>::new(&mut g);
        j.calculate_shortest_paths();
    }

}
