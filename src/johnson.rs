extern crate two_d_array;
use std::collections::{BTreeMap};
//use two_d_array::TwoDArray;

use crate::dirgraph::DirectedGraph;
use crate::graphbuilder::GraphBuilder;
use crate::ShortestPathInfo;

use log::{ info, error, debug /*,warn */,  trace };

// use std::fmt;
use crate::bellman::{Bellman,MinMax};
use crate::dijkstra::Dijkstra;
use crate::bellman::MinMax::{Value};

#[derive(Debug)]
pub struct Johnson<'a> {
        graph : &'a mut DirectedGraph,
        shortest_path_info:  BTreeMap::<usize,BTreeMap::<usize,ShortestPathInfo>>,
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
            shortest_path_info:  BTreeMap::<usize,BTreeMap::<usize,ShortestPathInfo>>::new(),
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
                let mut results = d.get_shortest_paths();
                info!("Results for Starting Vertex {} AFTER adjustment correction", start);
                for (vertex_id, info) in results.iter_mut() {
                    let mut updated_info = info.clone();
                    updated_info.distance = info.distance - adjustment_results[&start] + adjustment_results[vertex_id];
                    *info = updated_info
                }
                trace!("{:#?}",results);
                self.shortest_path_info.insert(start,results);

            }

        }
        else {
            info!("Found Negative Cycle")
        }

    }

    pub fn has_negative_cycle(&self) -> bool {
        self.found_negative_cycle
    }
    
        
    pub fn results_iter(&self) -> std::collections::btree_map::Iter<'_, usize, BTreeMap::<usize,ShortestPathInfo>>
    {
        self.shortest_path_info.iter()

    }

    pub fn shortest_shortest_path(&self) -> (MinMax<i64>, Vec<usize>){
        let mut min_path_len = MinMax::NA;
        let mut min_path = Vec::<usize>::new();
        for (_start, path_info) in self.shortest_path_info.iter() {
            for (_dest, info) in path_info.iter() {
                if info.distance < min_path_len {
                    min_path_len = info.distance;
                    min_path = info.path.clone();
                }

            }

        }
        (min_path_len, min_path)
        
    }



    /// Find all shortest path from each vertex to all other vertes
    pub fn find_shortest_shortest_path(&mut self) -> Option<ShortestPathInfo> {
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

            let mut shortest_distance = MinMax::Max;
            let mut shortest_path_info = None;

            for start in 1..self.num_vertex {
                debug!("Shortest distance now {}",shortest_distance);
                let mut d = Dijkstra::new(start);

                for (id, _v) in g_prime.vertex_iter() {
                    d.initialize_vertex(id.clone());
                }
                d.calculate_shortest_paths(&g_prime, start);

                if let Some(new_short_path_info) = d.get_shortest_shortest_path(shortest_distance,adjustment_results.clone()) {
                    if new_short_path_info.distance < shortest_distance {
                        info!("Found shorter path from {} dist {}",start,new_short_path_info.distance);
                        shortest_distance = new_short_path_info.distance;
                        shortest_path_info = Some(new_short_path_info);
                    }
                }
            }

            /*
            if ! shortest_path_info.is_none() {
                let unadjusted = shortest_path_info.unwrap();
                let mut new_info = unadjusted.clone();
                new_info.distance = unadjusted.distance - adjustment_results[&unadjusted.source] + adjustment_results[&unadjusted.dest];
                shortest_path_info = Some(new_info);
            }
            */
            shortest_path_info

        }
        else {
            info!("Found Negative Cycle");
            None
        }

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
