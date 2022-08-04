//use std::process; use std::io::{self, Write}; // use std::error::Error;
//use std::cmp;
use std::collections::{BTreeMap, BTreeSet};
use log::{ /* info */ error, /* debug,*/ warn, trace };

use std::fmt::Display; 
use std::fmt;

#[derive(Debug,Clone)]
pub struct Edge {
    edge_id: u32,
    source:  u32,
    dest:    u32,
    weight:  i64,
}

impl Display for Edge {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ({} -> {} w{})", self.edge_id, self.source, self.dest, self.weight)
    }

}

impl Edge {

    pub fn new(new_edge_id: u32, source_vertex_id: u32, dest_vertex_id: u32, weight: i64 ) -> Edge {
        trace!("New Edge {} from {} to {} with weight {}",new_edge_id,source_vertex_id,dest_vertex_id,weight);
        Edge {
            edge_id:    new_edge_id,
            source:     source_vertex_id,
            dest:       dest_vertex_id,
            weight:     weight,
        }
    }


    pub fn source(&self) -> u32 {
        self.source
    }

    pub fn dest(&self) -> u32 {
        self.dest
    }

    pub fn weight(&self) -> i64 {
        self.weight
    }
}



#[derive(Debug, Clone)]
pub struct Vertex {
	vertex_id: u32,
    // set of incomin and outgoing edge ids
	incoming: BTreeSet<u32>,
	outgoing: BTreeSet<u32>,
}

impl Vertex {

	pub fn new(id : u32) -> Vertex {
		let incoming = BTreeSet::<u32>::new();
		let outgoing = BTreeSet::<u32>::new();
		Vertex {vertex_id: id, 
				incoming: incoming, 
				outgoing: outgoing,
				}
	}

    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Vertex {}", self.vertex_id)
    }
	
	pub fn add_outgoing(&mut self, edge_id: u32) {
        trace!("Adding outgoing edge {} to vertex {}",edge_id, self.vertex_id);
        if !self.outgoing.insert(edge_id) {
           error!("add_outgoing: Vertex {} - outgoing edge {} already exists",edge_id, self.vertex_id)
        }
	}

	pub fn delete_outgoing (&mut self, edge_id: u32) {
        if !self.outgoing.remove(&edge_id) {
           error!("delete_outgoing:  Vertex {} - outgoing edge {} doesn't exist",self.vertex_id,edge_id)
        }
	}

	pub fn add_incoming(&mut self, edge_id: u32) {
        trace!("Adding incoming edge {} to vertex {}",edge_id,self.vertex_id);
        if !self.incoming.insert(edge_id) {
           error!("add_incoming: Vertex {} - outgoing edge {} already exists",self.vertex_id,edge_id)
        }
	}

	pub fn delete_incoming (&mut self, edge_id: u32) {
        if !self.incoming.remove(&edge_id) {
           error!("delete_incoming:  Vertex {} - outgoing edge {} doesn't exist",self.vertex_id,edge_id)
        }
	
	}

    pub fn get_outgoing_edges(&self)  -> Vec<u32>{
        // get the list of outgoing edges and map them to the dest vertex
		self.outgoing.iter().cloned().collect()
    }

    pub fn get_incoming_edges(&self)  -> Vec<u32>{
        // get the list of outgoing edges and map them to the dest vertex
		self.incoming.iter().cloned().collect()
    }

    pub fn id(&self) -> u32 {
        self.vertex_id
    }
}


#[derive(Debug,Clone)]
pub struct DirectedGraph {
	vertex_map:  BTreeMap::<u32, Vertex>,
    edge_map:   BTreeMap::<u32, Edge>,
    next_edge_id:  u32
}


impl DirectedGraph {
	pub fn new() -> DirectedGraph {
		let v_map = BTreeMap::<u32, Vertex>::new();
		let e_map = BTreeMap::<u32, Edge>::new();
		DirectedGraph {
				vertex_map:     v_map,
				edge_map:       e_map,
                next_edge_id:   1,
		}
	}

	pub fn define_vertex(&mut self, id: u32) -> Option<usize> {

		if self.vertex_map.contains_key(&id) {
			None
		} 
		else { 
            trace!("Adding Vertex {}",id);
			let v = Vertex::new(id.clone());
			self.vertex_map.insert(id,v);
			Some(self.vertex_map.len())  
		}
    }

	pub fn define_edge(&mut self, source: u32, dest: u32, weight: i64 ) -> Option<u32> {
        if source != 0 && dest != 0 {
            let edge_id = self.next_edge_id.clone();
            self.next_edge_id += 1;
			let e = Edge::new(edge_id, source, dest, weight);
			self.edge_map.insert(edge_id,e);
            Some(edge_id)
        }
        else {
            warn!("Invalid edge input 0  source {} dest {} weight {}", source, dest, weight);
            None
        }
	}

	pub fn add_edge(&mut self, v1: u32, v2: u32, weight: i64) {

		//create the vertexes, if the don't exist
		self.define_vertex(v1.clone());
		self.define_vertex(v2.clone());
        if let Some (edge_id) = self.define_edge(v1.clone(),v2.clone(),weight) {
            let v_map = &mut self.vertex_map;

            // add the edge to the first vertex's adjacency outgoing list
            let vert1 = v_map.get_mut(&v1).unwrap();
            vert1.add_outgoing(edge_id);

            // add the edge to the second vertex adjacency incoming list
            let vert2 = v_map.get_mut(&v2).unwrap();
            vert2.add_incoming(edge_id);
        }
        else {
            error!("Error adding Edge  v1 {} v2 {} w {}",v1,v2,weight);
        }

	}


	pub fn get_outgoing_vertex(&self, vertex: u32) -> Vec<Edge>{
		let v = self.vertex_map.get(&vertex).unwrap();
        // get the list of outgoing edges
        // by mapping each id to its dest element
        // NOTE: since edge list is coming from the vertex, this isn't handling the case where edge_map.get
        // returns 'None' ; this shouldn't occur, and will crash here if it did
		v.get_outgoing_edges().iter().map(|x| self.edge_map.get(&x).unwrap().clone()).collect()
		
	}

	pub fn get_incoming_vertex(&self, vertex: u32) -> Vec<Edge>{
		let v = self.vertex_map.get(&vertex).unwrap();
        // get the list of outgoing edges
        // by mapping each id to its dest element
        // NOTE: since edge list is coming from the vertex, this isn't handling the case where edge_map.get
        // returns 'None' ; this shouldn't occur, and will crash here if it did
		v.get_incoming_edges().iter().map(|x| self.edge_map.get(&x).unwrap().clone()).collect()
		
	}

    pub fn vertex_iter(&self) -> std::collections::btree_map::Iter<'_, u32, Vertex> {
        self.vertex_map.iter()
    }

	pub fn get_vertexes(&self) -> Vec<u32> {
		self.vertex_map.keys().cloned().collect()
	}

	pub fn print_vertexes(&self) {
        println!("Vertexes:");
		for (key, value) in &self.vertex_map {
//			let out_list : String = value.outgoing.iter().map(|x| {let e = self.edge_map.get(x).unwrap(); format!("e{} v{}(w{}) ; ",x,e.dest,e.weight) }).collect();
			let out_list : String = value.outgoing.iter().map(|x| {let e = 
                    self.edge_map.get(x)
                        .unwrap_or(&Edge { edge_id: 0,source: 0, dest: 0, weight: 0 }); format!("{} ; ",e) })
                        .collect();
			println!("Vertex {} ({}) :  outgoing list: {}",key,value.vertex_id,out_list);
		}
        println!("Edges");
        for (key, value) in &self.edge_map {
            println!("Edge id {}   {:?}", key, value);
        }


					
	}

	pub fn delete_edge(&mut self,edge_id: u32) -> Result<(),String>  {
	
        if let Some(edge) = self.edge_map.get(&edge_id) {
            self.vertex_map.get_mut(&edge.source).unwrap().delete_outgoing(edge_id)	;
            self.vertex_map.get_mut(&edge.dest).unwrap().delete_incoming(edge_id);
            self.edge_map.remove(&edge_id);
            Ok(())
        }
        else {
            error!("delete edge:  No such edge {}",edge_id);
            Err("Delete Edge: No such edge".to_string())
        }

	}
}




// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

	fn setup_basic1() -> DirectedGraph {
		let mut g = DirectedGraph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.add_edge(2,4),Some(2));
		assert_eq!(g.add_edge(3,4),Some(1));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.get_outgoing(4),&[]);
		g
	} 

    #[test]
    fn basic() {
		let mut g = DirectedGraph::new();
		assert_eq!(g.create_vertex(&1),Some(1));
		assert_eq!(g.create_vertex(&2),Some(2));
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2));
		assert_eq!(g.create_vertex(&3),Some(3));
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.add_edge(2,3),Some(1));
		assert_eq!(g.get_vertexes(),vec!(1,2,3));
		assert_eq!(g.add_edge(1,4),Some(3));
		assert_eq!(g.get_vertexes(),vec!(1,2,3,4));
		println!("{:?}",g);

    }

	#[test]
	fn test_add() {
		let mut g = DirectedGraph::new();
		assert_eq!(g.add_edge(1,2),Some(1));
		assert_eq!(g.get_outgoing(1),&[2]);
		assert_eq!(g.get_incoming(2),&[1]);
		assert_eq!(g.add_edge(1,3),Some(2));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_incoming(2),&[1]);
	}

	#[test]
	fn test_add_del() {
		let mut g = setup_basic1();
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.add_edge(1,2),Some(3));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.get_outgoing(2),&[3,4]);
		assert_eq!(g.get_outgoing(3),&[4]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[2,3]);
		assert_eq!(g.delete_edge(1,2),Ok(()));
		assert_eq!(g.get_outgoing(1),&[3]);
		
	}


}
