use std::env; use std::process; use std::io::{self, Write}; // use std::error::Error;
//use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap,BTreeMap};
use std::thread;
use log::{ info, /*error, */ debug, /*warn,*/ trace };

mod dirgraph;
use crate::dirgraph::DirectedGraph;


fn main() {

    env_logger::init();
    info!("Logging started");


    let args: Vec<String> = env::args().collect();

	println!("Args {:?} {}",args,args.len());

	if args.len() < 2 { eprintln!("Usage: {} filename <count>", args[0]); process::exit(1); }

  // Create a path to the desired file
    let path = Path::new(&args[1]);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

	let mut g = DirectedGraph::new();

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();
		let mut tokens = line_data.split_whitespace();
		let vertex = tokens.next().unwrap().parse::<u32>().unwrap();
		let adjacent : Vec<u32> = tokens.map(|x| x.to_string().parse::<u32>().unwrap()).collect();
		

		let mut other : u32 = 0;
//		g.create_vertex(&vertex);
		for other_v in &adjacent {

			other = other_v.clone();
			let _num_edges = g.add_edge(vertex,*other_v,1);
		
		}
		if _count % 100000 == 0 {
			println!(" {} : {}  from {}  to {}" ,_count,line_data,vertex,other);
			io::stdout().flush().unwrap();
		}
		if _count % 10000 == 0 {
			print!(".");
			io::stdout().flush().unwrap();
		} 
    }
	let child = thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(move || { 
	   // code to be executed in thread

		println!("Read {} lines",_count);
	//	g.print_vertexes();
	//	g.dfs_incoming(1,1,0);
	//	println!("Finish Order {:?}", g.finished_order);
	//	println!("Starting Vertex {:?}", g.start_search);
		g.finished_order = Vec::<u32>::new();
		g.start_search = HashMap::<u32,Vec::<u32>>::new();
		g.explored = HashMap::<u32,bool>::new();
		let list : Vec<u32> = g.vertex_map.keys().cloned().collect();
		g.dfs_loop_incoming(&list);
	//	println!("Finish Order {:?}", g.finished_order);
	//	println!("Starting Vertex {:?}", g.start_search);
		let list : Vec<u32> = g.finished_order.iter().rev().cloned().collect();
		g.dfs_loop_outgoing(&list);
		println!("\n Start search has {} entries",g.start_search.len());
		// println!("\n Start search {:?} entries",g.start_search);
		println!("\n Top Counts {:?} entries",g.top_search_cnts);
		let mut top_search_count_vec : Vec::<(u32, usize)> = g.top_search_cnts.iter().map(|(k,v)| (*k, *v)).collect();
		top_search_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
		println!("\n Top Counts {:?} entries",top_search_count_vec);
	}).unwrap(); 
	child.join().unwrap();
//	println!("Starting Vertex {:?}", g.start_search);
}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

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
