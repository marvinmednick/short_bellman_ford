extern crate two_d_array;
//use std::collections::{BTreeMap};
use two_d_array::TwoDArray;

use crate::dirgraph::DirectedGraph;
use crate::graphbuilder::GraphBuilder;

use log::{ info, error, /* debug ,*/ warn, /* trace*/ };

// use std::fmt;
use crate::bellman::{Bellman,MinMax};
//use crate::bellman::MinMax::{Value,NA};


pub struct Johnson<'a> {
        graph : &'a mut DirectedGraph,
        g_prime: DirectedGraph,
        adjustments:  Bellman,
        shortest_path_length: TwoDArray<MinMax<usize>>,
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

        let adjustment_info = Bellman::new(num_vertex.clone());

        Johnson {
            graph    : graph,
            g_prime  : DirectedGraph::new(),
            adjustments : adjustment_info,
            shortest_path_length : TwoDArray::<MinMax<usize>>::new(num_vertex,num_vertex,MinMax::NA),
            found_negative_cycle : false,
            num_vertex:  num_vertex,
        }

    }

    /// Find all shortest path from each vertex to all other vertes
    pub fn calculate_shortest_paths(&mut self) {
        info!("Starting all shortest path analysis with Johnson algorithm");
        self.found_negative_cycle = false;


        info!("Staring Bellman");
        self.adjustments.calculate_shortest_paths(self.graph, 0);
        // get the results of the bellman by vertex...  (need to change the way results are
        // reported)


        // creat a new graph with edges based on adjustment values.
        //
        //
        //
        // Run dijkstra on this graph n times
        //
        // Done... and report results

    }

    pub fn has_negative_cycle(&self) -> bool {
        self.found_negative_cycle
    }
    
        
    pub fn print_result(&self, display_list: Vec<usize>, short_display: bool) {

        warn!("print_result - TODO");
        println!("print_result - TODO");

        /*
        let mut is_first = true;

        if display_list.len() > 0 {
            for v in display_list {
                if v < self.num_vertex {
                   if !is_first {
                       print!(",");
                    }
                   is_first = false;
                   Bellman::print_vertex_result(v, self.distances.get(v,self.last_iteration).unwrap(),short_display);
                }
                else {
                    error!("Dest Vertex {} is invalid",v);
                }
            }
            println!();

        }
        else {
            let mut index = 0;
            for result in self.distances.get_row(self.last_iteration) {
               if !is_first {
                   print!(",");
                }
               is_first = false;
               Johnson::print_vertex_result(index, *result, short_display);
               index += 1;
            } 
            println!();
        }
        */

    }

    fn print_vertex_result(vertex: usize, result: MinMax<i64>, short: bool) {

        if short {
            print!("{}", result);
        }
        else {
            println!("v {} - {}", vertex, result);
        }

    }

/*

    fn find_path(&self,starting_vertex: usize, dest_vertex: usize) -> Vec<usize>{

        info!("Finding path for vertex {}", dest_vertex);
        let mut vertex_list = Vec::<usize>::new();
        let mut predecessor_count = 0;
        // put the destination vertex at the end of the list to match the stanford test cases
        // (doesn't seem correct to me)
        vertex_list.push(dest_vertex);
        
        // unless this vertex doesn't have a predecessor (indicating no path from starting vertex)
        // add it to the end of the path
        if self.predecessor[&dest_vertex] != NA || dest_vertex == self.starting_vertex {

            let mut current_vertex = dest_vertex;
            let mut done = false;
            while self.predecessor[&current_vertex] != NA && !done {
                predecessor_count += 1;
                if let Value(preceeding_vertex) = self.predecessor[&current_vertex] {
                    trace!("Adding Vertex {} to the path", preceeding_vertex);
                    vertex_list.push(preceeding_vertex.clone());
                    current_vertex = preceeding_vertex;
                }
                else {
                    error!("Unexpected Value");
                }
                if predecessor_count > self.num_vertex || current_vertex == self.starting_vertex {
                    done = true;
                }

             }
        }
        let path : Vec<usize> = vertex_list.into_iter().rev().collect();
        info!("Path from vertex {} to vertex {} -> {:?}", starting_vertex, dest_vertex, path);
        path

    }

    */
    pub fn print_paths(&self, vertex_list: Vec<usize>) {

        warn!("print_paths TODO");
        println!("print_paths TODO");
        /*
        for start in vertex_list {
            for v in self.graph.vertexes() {
                let path = self.find_path(start, v);
            }
            let has_cycle = path.len() > self.num_vertex;

            let mut first=true;
            let path_string : String = path.iter().map( |v| { if first { first=false; format!("{}",v) } else { format!(", {}",v) } } ).collect();

            print!("{} => path => {}",v,path_string);
            if has_cycle {
                print!("... (has cycle)");
            }
            println!();
        }
        */
    }


}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

	fn setup_basic() -> Johnson<'static> {
		let mut g = DirectedGraph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.add_edge(2,4),Some(2));
		assert_eq!(g.add_edge(3,4),Some(1));
		assert_eq!(g.get_outgoing_vertex_ids(1),&[2,3]);
		assert_eq!(g.get_outgoing_vertex_ids(2),&[3,4]);
		assert_eq!(g.get_outgoing_vertex_ids(3),&[4]);
		assert_eq!(g.get_outgoing_vertex_ids(4),&[]);
        Johnson::new(&mut g)
	} 

    #[test]
    fn basic() {
        let j = setup_basic(); 
        println!("{:#?}",j);
    }

}
