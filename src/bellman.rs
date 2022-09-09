extern crate two_d_array;
use std::collections::{BTreeMap};
use two_d_array::TwoDArray;

use crate::dirgraph::DirectedGraph;

use log::{ info, error, debug, /*warn,*/ trace };

use std::fmt;
use crate::bellman::MinMax::{Value,NA};

#[derive(Debug,Clone,Copy,PartialOrd,Ord,PartialEq,Eq)]
pub enum MinMax<T> {
    Min,
    Value(T),
    Max,
    NA,
}

// Implement `Display` for `MinMax`.
impl<T: fmt::Display> fmt::Display for MinMax<T>

{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MinMax::Min => f.pad(&format!("Min")),
            MinMax::Max => f.pad(&format!("Max")),
            MinMax::NA =>  f.pad(&format!("NA")),
            MinMax::Value(ref x) =>  f.pad(&format!("{}", x))
        }
    }
}


pub struct Bellman {
        /// Two dimensional array containing the length of the shortest path to each vertex for
        /// each iteration (size is )
        distances:  TwoDArray<MinMax<i64>>,
        /// For each vertex the precessor contains the preceeding vertex in the path.  The vertexes
        /// of shortest path can be found by traversing the precessor vertexes back to the source
        predecessor:  BTreeMap<usize,MinMax<usize>>,
        // number of vertexes is set to 1 more than the actually number of vertex to simplify having vertex numbers starting at one without a
        // mapping.  Vertex 0 is unused
        num_vertex: usize,
        // number of iterations -- genearlly set to the same as the number of vertexes to allow
        // detection of negative cycles.  (Note that this also supports vertex 0 even if the first
        // vertex id is 1 as not to have to map from vertex ids to indexes.) 
        iterations: usize,
        last_iteration: usize,
        starting_vertex: usize,
        found_negative_cycle : bool,
}


impl Bellman {

    pub fn new(num_vertex: usize ) -> Bellman {
        let width = num_vertex+1;
        let height = num_vertex+1;
        let mut preceeding = BTreeMap::<usize,MinMax<usize>>::new();


        // initialize the preceeding vertex to Max indicating no path yet
        for id in 0..width {
            preceeding.insert(id,NA);
        }

        Bellman { 
            distances:  TwoDArray::<MinMax<i64>>::new(width,height,MinMax::Max),
            predecessor:  preceeding,
            num_vertex: width,
            iterations: height,
            last_iteration: 0,
            starting_vertex: 0,
            found_negative_cycle: false,
        }

    }

    /// Find the shortest path from a starting vertex to all other vertexes in the graph
    pub fn calculate_shortest_paths(&mut self, graph: &DirectedGraph, starting_vertex: usize) {
        info!("Starting shortest path with {}",starting_vertex);
        self.found_negative_cycle = false;

        // initialite the first iteration with the distance from the starting
        // vertex to itself as 0 -- all other items will be left at none
        self.distances.set(starting_vertex,0,MinMax::Value(0));
        self.starting_vertex = starting_vertex;


        // vertex ids start at 1.
        for iteration in 1..self.iterations {
            info!("Iteration {}",iteration);
            let mut changes_during_iteration = false;
            for (id,_v) in graph.vertex_iter() {
                let edges = graph.get_incoming(*id);

                let mut incoming_distances = Vec::<(MinMax<i64>,usize)>::new();
                for e in edges {
                    if let Ok(dist) = self.distances.get(e.source(),iteration-1) {
                        if let Value(edge_distance) = dist {
                            if dist < MinMax::<i64>::Max {
                                //push a tuple with the new weight as primary element and source vertex
                                //as 2nd
let this_distance = MinMax::Value(edge_distance + e.weight());
                                let this_entry = (this_distance.clone(),e.source());
                                debug!("Adding {} from {} {:?}",this_distance, e.source(),this_entry);
                                incoming_distances.push(this_entry);
                            }
                        }
                    }
                }

                // start with the values from the last iteration
                let last = self.distances.get(*id,iteration-1).unwrap();
                debug!("Vertex {} last iteration value was {}",id,last);

                let mut new = last;
                // mark the source as the current node (indicating we are taking the last value)
                let mut source = id.clone();

                if incoming_distances.len() > 0 {
                    // find the min of the incoming distances (which are in a tuple)
                    debug!("Incoming option {:?}",incoming_distances);
                    let (incoming_min, incoming_source) = incoming_distances.iter().min().unwrap();
                    

                    //check to see if the incoming value is less, if so update the fields from the
                    //tuple
                    if *incoming_min < new {
                        new = *incoming_min;
                        source = *incoming_source;
                        changes_during_iteration = true;
                        self.predecessor.insert(*id, Value(source.clone()));

                    }
                }
                else {
                    trace!("Vertex {} - no incoming edges",id);
                }
                // set the new value
                self.distances.set(*id,iteration,new);
                debug!("Vertex {} iter: {} last {:?} cur via {} -> {:?} new {:?}",id,iteration,last,source,last,new);

            }
            self.last_iteration += 1;
            if !changes_during_iteration {
                info!("No changes during iteration {} ... finishing", iteration);
                break;
            }
            else if iteration == self.iterations-1 {
                info!("changes deteted in {} iteration --- Graph has a negative cycle", iteration);
                self.found_negative_cycle = true;

            }
        }

        let mut count = 0;
        let header : String = (0..self.num_vertex).map(|index| format!("{:>4} ",index) ).collect();
        trace!("{:13}{}","Vertex",header);
            

        for row in self.distances.get_row_iter() {
            let min = row.iter().min().unwrap();
            let row_format : String = row.iter().map(|val| format!("{:>4} ",val) ).collect();
            info!("Iter {:2} :    {}  Min: {}", count,row_format, min);
            count += 1

        }

    }

    pub fn has_negative_cycle(&self) -> bool {
        self.found_negative_cycle
    }
   
    /// Returns the shortest disntance calcuated from the starting vertex previously defined
    /// to the dest_vertex provided. 
    /// Returns NA if the dest_vertex is out of rant
    pub fn get_shortest_path_distance (&self,dest_vertex: usize ) -> MinMax<i64> {
        if dest_vertex < self.num_vertex {
            self.distances.get(dest_vertex, self.last_iteration).unwrap()
        }
        else {
            MinMax::NA
        }
    }
        
    /// Returns the a list of all the hortest disntance calcuated from the starting vertex
    /// to each of the rest of the vertexes 
    pub fn get_shortest_path_distances(&self) -> Vec<MinMax<i64>> {

        let mut result_list = Vec::<MinMax<i64>>::new();
        for result in self.distances.get_row(self.last_iteration) {
            result_list.push(result.clone());
        }

        result_list

    }


    pub fn print_result(&self, display_list: Vec<usize>, short_display: bool) {
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
               Bellman::print_vertex_result(index, *result, short_display);
               index += 1;
            } 
            println!();
        }

    }

    fn print_vertex_result(vertex: usize, result: MinMax<i64>, short: bool) {

        if short {
            print!("{}", result);
        }
        else {
            println!("v {} - {}", vertex, result);
        }

    }


    fn find_path(&self, dest_vertex: usize) -> Vec<usize>{

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
        info!("Path from vertex {} to vertex {} -> {:?}", self.starting_vertex, dest_vertex, path);
        path

    }

    pub fn get_shortest_paths(&self, vertex_list: Vec<usize>) -> Vec<(usize, Vec<usize>,bool)> {
        let mut result = Vec::<(usize,Vec<usize>,bool)>::new();
        for v in vertex_list {
            let path = self.find_path(v);
            let has_cycle = path.len() > self.num_vertex;
            result.push((v,path,has_cycle));
            
        }
        result

    }

    pub fn print_paths(&self, vertex_list: Vec<usize>) {

        for v in vertex_list {
            let path = self.find_path(v);
            let has_cycle = path.len() > self.num_vertex;

            let mut first=true;
            let path_string : String = path.iter().map( |v| { if first { first=false; format!("{}",v) } else { format!(", {}",v) } } ).collect();

            print!("{} => path => {}",v,path_string);
            if has_cycle {
                print!("... (has cycle)");
            }
            println!();
        }
    }

}
