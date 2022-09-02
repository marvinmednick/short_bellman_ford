extern crate minheap;
use std::collections::{HashMap,BTreeMap};
use minheap::MinHeap;

use crate::dirgraph::DirectedGraph;

use log::{ info, error, debug, /*warn,*/ trace };

pub struct Dijkstra {
        explored:  HashMap::<usize,bool>,
        unprocessed_vertex : MinHeap::<i64>,
        processed_vertex : BTreeMap::<usize,i64>,
}


impl Dijkstra {

    pub fn new() -> Dijkstra {
        Dijkstra { 
            explored:  HashMap::<usize,bool>::new(),
            unprocessed_vertex : MinHeap::<i64>::new(),
            processed_vertex : BTreeMap::<usize,i64>::new(),
        }

    }

    pub fn initialize_vertex(&mut self, vertex_id: usize) {
        self.unprocessed_vertex.insert(vertex_id,100000000);
    }
        

    pub fn shortest_paths(&mut self, graph: &DirectedGraph, starting_vertex: usize) {
        info!("Starting shortest path with {}",starting_vertex);

        if let Some(starting_index) = self.unprocessed_vertex.get_id_index(starting_vertex) {

            let index = starting_index.clone();
            self.unprocessed_vertex.delete(index);
            
            // setup the initial distance for the starting vertex to 0 (to itself)
            self.processed_vertex.insert(starting_vertex,0);

            self.update_scoring(graph, starting_vertex);

            while let Some((next_vertex,next_vertex_score)) = self.unprocessed_vertex.get_min_entry() {
                debug!("Processing vertex {} score: {}",next_vertex,next_vertex_score);
                self.processed_vertex.insert(next_vertex,next_vertex_score);
                self.update_scoring(graph, next_vertex);
            }
         }       
        else {
            error!("Starting vertex {} is not in the graph",starting_vertex);
        }

    }

    // Update scoring in the unprocessed pool of vertexes related to 
    // vertex of id.
    fn update_scoring(&mut self, graph: &DirectedGraph, cur_vertex: usize) {
        debug!("Dijsktra scoring for vertex {}",cur_vertex);

        // get the list of edge that are outgoing from the current vertex
        let adj_edges = graph.get_outgoing(cur_vertex);
        
        // get the distance/score of the current vertex as a start
        let cur_vertex_distance = self.processed_vertex.get(&cur_vertex).unwrap().clone();

        // update each of this nodes adjancent vertexes, if the new distance
        // is < the current distance
        for e in adj_edges {
            debug!("Dijsktra updating adjacent {:?}",e);
            // if the adjacent vertex is still in the unprocessed list, then 
            // update the scoring, otherwise skip it (since its already in the processed list)
            if let Some(cur_score) = self.unprocessed_vertex.peek_id_data(e.dest()) {
                let new_score = cur_vertex_distance + e.weight();
                if new_score < cur_score {
                    trace!("Update scoring on {} from {} to {}",e.dest(),cur_score,new_score);
                    // get the index of the item id
                    let vertex_index = self.unprocessed_vertex.get_id_index(e.dest()).unwrap().clone();
                    // and update its value
                    self.unprocessed_vertex.update(vertex_index,new_score);
                    trace!("Unprocessed: {:?}",self.unprocessed_vertex);
                }
             }       
            
        }

    }

    pub fn get_processed(&self,index : &usize) -> i64 {
        self.processed_vertex[index]
    }


    pub fn print_result(&self, display_list: Vec<usize>, short_display: bool) {
        let mut is_first = true;
        if display_list.len() > 0 {
            for v in display_list {
                if is_first { is_first = false; } else { print!(","); }
                if self.processed_vertex.contains_key(&v) {
                    Dijkstra::print_vertex_result(v, self.get_processed(&v),short_display);
                }
                else {
                    error!("Dest Vertex {} is invalid",v);
                }
            }
            println!();

        }
        else {
            for (v, result) in self.processed_vertex.iter() {
                Dijkstra::print_vertex_result(*v, *result,short_display);
            }
            println!();
        }

    }

    fn print_vertex_result(vertex: usize, result: i64, short: bool) {

        if short {
            print!("{}", result);
        }
        else {
            println!("v {} - {}", vertex, result);
        }

    }


}
