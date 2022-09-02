extern crate two_d_array;
use std::collections::{HashMap,BTreeMap};
use two_d_array::TwoDArray;

use crate::dirgraph::DirectedGraph;

use log::{ info, error, debug, /*warn,*/ trace };

use std::fmt;
use crate::bellman::MinMax::Value;

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
        distances:  TwoDArray<MinMax<i64>>,
        num_vertex: usize,
        iterations: usize,
}


impl Bellman {

    pub fn new(num_vertex: usize ) -> Bellman {
        let width = num_vertex+1;
        let height = num_vertex;

        Bellman { 
            distances:  TwoDArray::<MinMax<i64>>::new(width,height,MinMax::Max),
            num_vertex: width,
            iterations: height,
        }

    }

    /// Find the shortest path from a starting vertex to all other vertexes in the graph
    pub fn shortest_paths(&mut self, graph: &DirectedGraph, starting_vertex: usize) {
        info!("Starting shortest path with {}",starting_vertex);

        // initialite the first iteration with the distance from the starting
        // vertex to itself as 0 -- all other items will be left at none
        self.distances.set(starting_vertex,0,MinMax::Value(0));


        for iteration in 1..self.iterations {
            info!("Iteration {}",iteration);
            for (id,v) in graph.vertex_iter() {
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
                    }
                }
                else {
                    trace!("Vertex {} - no incoming edges",id);
                }
                // set the new value
                self.distances.set(*id,iteration,new);
                debug!("Vertex {} iter: {} last {:?} cur via {} -> {:?} new {:?}",id,iteration,last,source,last,new);
            }
        }

        let mut count = 0;
        for row in self.distances.get_row_iter() {
            let min = row.iter().min().unwrap();
            let row_format : String = row.iter().map(|val| format!("{:>4} ",val) ).collect();
            println!("Iter {:2} :    {}  Min: {}", count,row_format, min);
            count += 1

        }

    }
        

    pub fn print_result(&self, display_list: Vec<usize>, short_display: bool) {
        let mut is_first = true;
        if display_list.len() > 0 {
            for v in display_list {
                if is_first { is_first = false; } else { print!(","); };
                    println!("TODO");
                   /*Bellman::print_vertex_result(v, self.distances.get(&v,),short_display);
                }
                else {
                    error!("Dest Vertex {} is invalid",v);
                }
                */
            }
            println!();

        }
        else {
            println!("TODO");
           /* for (v, result) in self.processed_vertex.iter() {
                Bellman::print_vertex_result(*v, *result,short_display);
            } */
            println!();
        }

    }

    fn print_vertex_result(vertex: u32, result: MinMax<i64>, short: bool) {

        if short {
            print!("{}", result);
        }
        else {
            println!("v {} - {}", vertex, result);
        }

    }


}
