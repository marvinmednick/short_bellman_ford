use std::env; use std::process; use std::io::{self, Write}; // use std::error::Error;
//use std::cmp;
use std::collections::{HashMap,BTreeMap};

#[derive(Debug, Clone)]
struct Vertex {
	vertex_id: u32,
	incoming: BTreeMap<u32,u32>,
	incoming_cnt: usize,
	outgoing: BTreeMap<u32,u32>,
	outgoing_cnt: usize,
}

impl Vertex {

	pub fn new(id : &u32) -> Vertex {
		let incoming = BTreeMap::<(u32,i32),u32>::new();
		let outgoing = BTreeMap::<(u32,i32),u32>::new();
		Vertex {vertex_id: id.clone(), 
				incoming: incoming, 
				outgoing: outgoing,
				incoming_cnt : 0,
				outgoing_cnt : 0,
				}
	}
	
	pub fn add_outgoing(&mut self, vertex_id: u32, length: i32) {
		let counter = self.outgoing.entry((vertex_id,length).or_insert(0);
		*counter += 1;
		self.outgoing_cnt += 1;
	}

	pub fn del_outgoing (&mut self, vertex_id: u32, length: i32) ->  Result <(), String> {

		match self.outgoing.get_mut((&vertex_id,length)) {
			None | Some(0)  => Err("Invalid Vertex".to_string()),
			Some(1)        =>  	{ 	
									self.outgoing.remove(&(vertex_id,length); 
									self.outgoing_cnt -= 1;
									Ok(())
								}, 
			Some(x)        => 	{	*x -=1;  
								 	self.outgoing_cnt -= 1;
								 	Ok(())
								},
		}
	}

	pub fn add_incoming(&mut self, vertex_id: u32, length: i32) {
		let counter = self.incoming.entry((vertex_id,length)).or_insert(0);
		*counter += 1;
		self.incoming_cnt += 1;
	}

	pub fn del_incoming (&mut self, vertex_id: u32, length: i32) -> Result<(),String> {
	
		match self.incoming.get_mut(&(vertex_id,length)) {
			None | Some(0)  => Err("Invalid Vertex".to_string()),
			Some(1)        =>	{ 
									self.incoming.remove(&(vertex_id,length)); 
									self.incoming_cnt -= 1;
									Ok(())
								}, 
			Some(x)        => 	{
									*x -=1;
									self.incoming_cnt -= 1;
									Ok(())
								},
		}

	}
}


#[derive(Debug,Clone)]
struct DirectedGraph {
	vertex_map:  BTreeMap::<u32, Vertex>,
	edge_count:  u32,
}


impl DirectedGraph {
	pub fn new() -> DirectedGraph {
		let v_map = BTreeMap::<u32, Vertex>::new();
		DirectedGraph {
				vertex_map: v_map,
				edge_count: 0,
		}
	}


	pub fn get_outgoing(&self, vertex: u32) -> Vec<u32>{
		let v = self.vertex_map.get(&vertex).unwrap();
		v.outgoing.keys().cloned().collect()
		
	}

	pub fn get_incoming(&self,vertex: u32) -> Vec<u32> {
		let v = self.vertex_map.get(&vertex).unwrap();
		v.incoming.keys().cloned().collect()
		
	}


	pub fn get_vertexes(&self) -> Vec<u32> {
		self.vertex_map.keys().cloned().collect()
			
	}

	pub fn print_vertexes(&self) {
		for (key, value) in &self.vertex_map {
			let out_list : String = value.outgoing.iter().map(|(x, y)| if y > &1 {format!("{}({}) ; ",x,y) } else { format!("{} ;",x)}).collect();
			println!("Vertex {} ({}) :  {}",key,value.vertex_id,out_list);
		}
					
	}

	pub fn create_vertex(&mut self,id: &u32) -> Option<usize> {

		if self.vertex_map.contains_key(&id) {
			None
		} 
		else { 
			let v = Vertex::new(&id);
			self.vertex_map.insert(id.clone(),v.clone());
			Some(self.vertex_map.len())  
		}
	}


	pub fn add_edge(&mut self, v1: u32, v2: u32) -> Option<usize> {

		//create the vertexes, if the don't exist
		self.create_vertex(&v1);
		self.create_vertex(&v2);

		let v_map = &mut self.vertex_map;
		// add the edge to the first vertex's adjanceny list
		let vert = v_map.get_mut(&v1).unwrap(); 
		vert.add_outgoing(v2);
		let new_cnt = vert.outgoing_cnt.clone();

		// add the edge to the second vertex adjacentcy list
		let vert2 = v_map.get_mut(&v2).unwrap(); 
		vert2.add_incoming(v1);

		self.edge_count += 1;
		Some(new_cnt)

	}

	pub fn delete_edge(&mut self,v1 : u32, v2 : u32) -> Result<(),String>  {
	
		self.vertex_map.get_mut(&v1).unwrap().del_outgoing(v2)?	;
		self.vertex_map.get_mut(&v2).unwrap().del_incoming(v1)?;
		self.edge_count -= 1;
		Ok(())

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
