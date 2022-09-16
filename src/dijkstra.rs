extern crate minheap;
use std::collections::{BTreeMap};
use minheap::MinHeap;

use crate::dirgraph::DirectedGraph;

use log::{ /* info ,*/ error, debug, warn, trace };
use crate::bellman::{MinMax,MinMax::Value,MinMax::NA};
use crate::shortpathinfo::ShortestPathInfo;

#[derive(Debug,Clone,PartialOrd,PartialEq)]
pub struct VertexInfo {
    // first entry in the field, so will be used for sorting by min heap by default
    score: MinMax<i64>,
    preceeding_vertex: Option<usize>,
}


pub struct Dijkstra {
        /// Starting vertex for the algoritm
        starting_vertex:  usize,
        ///  Unprocessed vertex -- Min Heap based the greedy score for each vertex
        ///  initially set to a maximum value, and is reduced during processing
        unprocessed_vertex : MinHeap::<VertexInfo>,
        /// Processed Vertexes -- Map of all vertexes already processed, along with there distance
        /// from the starting vertex
        processed_vertex : BTreeMap::<usize,VertexInfo>,
        /// If set, vertex X  contains the preceeding vertex in the path from the starting vertex
        /// used to build the path from Start to this vertex
        predecessor:  BTreeMap<usize,Option<usize>>,
}


impl Dijkstra {

    pub fn new(starting_vertex: usize) -> Dijkstra {
        Dijkstra { 
            starting_vertex:  starting_vertex,
            unprocessed_vertex : MinHeap::<VertexInfo>::new(),
            processed_vertex : BTreeMap::<usize,VertexInfo>::new(),
            predecessor : BTreeMap::<usize,Option<usize>>::new()
        }

    }

    pub fn initialize_vertex(&mut self, vertex_id: usize) {
        self.unprocessed_vertex.insert(vertex_id,VertexInfo {  score: MinMax::Max, preceeding_vertex: None} );
        self.predecessor.insert(vertex_id,None);
    }
        

    pub fn calculate_shortest_paths(&mut self, graph: &DirectedGraph, starting_vertex: usize) {
        trace!("Starting shortest path calucation for Vertex {}",starting_vertex);

        if let Some(starting_index) = self.unprocessed_vertex.get_id_index(starting_vertex) {

            let index = starting_index.clone();
            self.unprocessed_vertex.delete(index);
            
            // setup the initial distance for the starting vertex to 0 (to itself) and no
            // associated Vertex
            self.processed_vertex.insert(starting_vertex,VertexInfo { score: Value(0), preceeding_vertex: None } );
            self.predecessor.insert(starting_vertex,None);

            self.update_scoring(graph, starting_vertex);

            while let Some((next_vertex,next_vertex_info)) = self.unprocessed_vertex.get_min_entry() {
                debug!("Processing vertex {} score: {}",next_vertex,next_vertex_info.score);
                self.processed_vertex.insert(next_vertex,next_vertex_info);
                self.update_scoring(graph, next_vertex);
            }
         }       
        else {
            warn!("Starting vertex {} is not in the graph",starting_vertex);
        }

    }

    // Update scoring in the unprocessed pool of vertexes related to 
    // vertex of id.
    fn update_scoring(&mut self, graph: &DirectedGraph, cur_vertex: usize) {
        debug!("Dijsktra scoring for vertex {}",cur_vertex);

        // get the list of edge that are outgoing from the current vertex
        let adj_edges = graph.get_outgoing_edges(cur_vertex);
        
        // get the distance/score of the current vertex as a start
        let cur_vertex_info = self.processed_vertex.get(&cur_vertex).unwrap().clone();
        let cur_vertex_distance = cur_vertex_info.score;

        // update each of this nodes adjancent vertexes, if the new distance
        // is < the current distance
        for e in adj_edges {
            debug!("Dijsktra updating adjacent {:?}",e);
            // if the adjacent vertex is still in the unprocessed list, then 
            // update the scoring, otherwise skip it (since its already in the processed list)
            if let Some(cur_info) = self.unprocessed_vertex.peek_id_data(e.dest()) {
                let new_score = cur_vertex_distance + Value(e.weight());
                if new_score < cur_info.score {
                    trace!("Update scoring on {} from {} to {}, cur_vertex is {} e.source {}",e.dest(),cur_info.score,new_score,cur_vertex, e.source());
                    // get the index of the item id
                    let vertex_index = self.unprocessed_vertex.get_id_index(e.dest()).unwrap().clone();
                    // and update its value
                    self.unprocessed_vertex.update(vertex_index,VertexInfo { score: new_score, preceeding_vertex: Some(cur_vertex)} );
                    self.predecessor.insert(e.dest(),Some(cur_vertex));
                    trace!("Unprocessed: {:?}",self.unprocessed_vertex);
                    trace!("Predecessors: {:?}",self.predecessor);
                }
             }       
            
        }

    }

    pub fn get_processed(&self,index : &usize) -> &VertexInfo {
        &self.processed_vertex[index]
    }


    /// Returns the shortest disntance calcuated from the starting vertex previously defined
    /// to the dest_vertex provided. 
    /// Returns NA if the dest_vertex is out of rant
    pub fn get_shortest_path_distance (&self,dest_vertex: usize ) -> MinMax<i64> {
        if self.processed_vertex.contains_key(&dest_vertex) {
            self.get_processed(&dest_vertex).score.clone()
        }
        else {
            NA
        }
    }
        
    /// Returns the a list of all the hortest disntance calcuated from the starting vertex
    /// to each of the rest of the vertexes 
    pub fn get_shortest_path_distances(&self) -> BTreeMap<usize, MinMax<i64>> {

        let mut result_list = BTreeMap::<usize,MinMax<i64>>::new();
        // add vertex 0 since it not define  (TODO -- cleanup vertex numbering and naming)
        result_list.insert(0,MinMax::Max);
        for (v, result) in self.processed_vertex.iter() {
            trace!("getsp_dist: v {} result {:?}",v,result);
            result_list.insert(*v,result.score.clone());
        }

        result_list

    }



    fn find_path(&self, dest_vertex: usize) -> Vec<usize>{

        trace!("Finding path for vertex {}", dest_vertex);
        let mut vertex_list = Vec::<usize>::new();
//        let mut predecessor_count = 0;
        // put the destination vertex at the end of the list to match the stanford test cases
        // (doesn't seem correct to me)
        vertex_list.push(dest_vertex);
        
        // unless this vertex doesn't have a predecessor (indicating no path from starting vertex)
        // add it to the end of the path
        if self.predecessor[&dest_vertex] != None || dest_vertex == self.starting_vertex {

            let mut current_vertex = dest_vertex;
            while self.predecessor[&current_vertex] != None  {
 //               predecessor_count += 1;
                if let Some(preceeding_vertex) = self.predecessor[&current_vertex] {
                    trace!("Adding Vertex {} to the path", preceeding_vertex);
                    vertex_list.push(preceeding_vertex.clone());
                    current_vertex = preceeding_vertex;
                }
                else {
                    error!("Unexpected Value");
                }

             }
        }
        let path : Vec<usize> = vertex_list.into_iter().rev().collect();
        trace!("Path from vertex {} to vertex {} -> {:?}", self.starting_vertex, dest_vertex, path);
        path

    }

    pub fn get_shortest_paths(&self) -> BTreeMap<usize, ShortestPathInfo> {
        let mut result = BTreeMap::<usize,ShortestPathInfo>::new();
        for (v, info) in self.processed_vertex.iter() {
            trace!("getsp_dist: v {} info {:?}",v,info);

            let path = self.find_path(*v);
            let path_len = path.len();

            let entry = ShortestPathInfo {
                source: self.starting_vertex,
                dest: *v,
                distance: info.score.clone(),
                path,
                path_len,
                has_negative_cycle : false,
            };
            debug!("info {:?}",entry);
            result.insert(*v,entry);
        }
        result

    }

    pub fn get_shortest_shortest_path(&self, max_distance: MinMax<i64>,adjustments: BTreeMap<usize,MinMax<i64>>) -> Option<ShortestPathInfo> {

        // set the currently found min distnces to the max allowed distance
        let mut min_distance = max_distance;
        let mut result = None;

        let mut _vertex_count = 0;
        for (v, info) in self.processed_vertex.iter() {
            trace!("Checking for shortest path from {} to {}",self.starting_vertex,v);
            _vertex_count += 1;
            let adjusted_distance = info.score - adjustments[&self.starting_vertex] + adjustments[v];

            if self.starting_vertex != *v && adjusted_distance < min_distance {

                let path = self.find_path(*v);
                let path_len = path.len();
                let new_entry = ShortestPathInfo {
                    source: self.starting_vertex,
                    dest: *v,
                    distance: adjusted_distance,
                    path,
                    path_len,
                    has_negative_cycle : false,
                };
                min_distance = adjusted_distance;
                trace!("Shorter path found: {}->{} w {} {:?}",new_entry.source,new_entry.dest, new_entry.distance, new_entry.path);
                result = Some(new_entry);
            }
        }
        result

    }
}

