extern crate two_d_array;
use std::collections::{HashMap,BTreeMap};
use two_d_array::TwoDArray;

use crate::dirgraph::DirectedGraph;

use log::{ info, error, debug, /*warn,*/ trace };

pub struct Bellman {
        distances:  TwoDArray<i64>,
        num_vertex: usize,
        iterations: usize,
}


impl Bellman {

    pub fn new(num_vertex: usize ) -> Bellman {
        let width = num_vertex+1;
        let height = num_vertex;

        Bellman { 
            distances:  TwoDArray::<i64>::new(width,height),
            num_vertex: width,
            iterations: height,
        }

    }

    /// Find the shortest path from a starting vertex to all other vertexes in the graph
    pub fn shortest_paths(&mut self, graph: &DirectedGraph, starting_vertex: usize) {
        info!("Starting shortest path with {}",starting_vertex);

        // initialite the first iteration with the distance from the starting
        // vertex to itself as 0 -- all other items will be left at none
        self.distances.set(starting_vertex,0,0);


        for iteration in 1..self.iterations {
            info!("Iteration {}",iteration);
            for (id,v) in graph.vertex_iter() {
                let edges = graph.get_incoming(*id);
                for e in edges {
                    //TODO -- need to find the min value from all the incoming edges
                    //and then compare that to the previous version

                    // get the last stance for this vertex
                    let last = self.distances.get(*id,iteration-1);

                    // calculuate the distance via the incoming edge
                    let cur = match self.distances.get(e.source(),iteration-1) {
                        // if the source destination has a distance then calculate it
                        Some(val) => Some(val + e.weight()),
                        // otherwise no path yet from the source dest
                        None => None
                    };
                    let new = {
                        if last == None {
                            cur
                        }
                        else if cur == None {
                            last
                        }
                        else if cur < last {
                            cur
                        }
                        else {
                            last
                        }
                    };
                    debug!("Vertex {} last {:?} cur via {} -> {:?} new {:?}",id,last,e.source(),cur,new);
                    if new != None {
                        self.distances.set(*id,iteration,new.unwrap());
                    }
                    
                }
            }
        }

        self.distances.log_display();

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

    fn print_vertex_result(vertex: u32, result: i64, short: bool) {

        if short {
            print!("{}", result);
        }
        else {
            println!("v {} - {}", vertex, result);
        }

    }


}
