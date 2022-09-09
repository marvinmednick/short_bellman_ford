use std::path::Path;
use std::fs::File;
use log::{  info , error, debug, /*warn, */trace };

use clap::Parser;


mod graphbuilder;

mod cmd_line;
use crate::cmd_line::CommandArgs;
use crate::cmd_line::Commands;

mod dirgraph;
use crate::dirgraph::DirectedGraph;

mod dijkstra;
use crate::dijkstra::Dijkstra;

mod bellman;
use crate::bellman::{Bellman,MinMax};

mod johnson;
use crate::johnson::Johnson;

mod parse;
use crate::parse::{read_adjacency_multi };

fn print_distance_result(results: Vec<MinMax<i64>>, display_list: Vec<usize>) {


//    trace!("Results {:?}",results);
//    for i in 0..10 {
////        trace!("Results {} -> {}",i,results[i]);
 //   }
    let disp_list_len = display_list.len().clone();
    let mut list_of_vertexes =  display_list;
    if disp_list_len == 0 {
        list_of_vertexes = (0..results.len()).collect();
    }

    let mut is_first = true;
    for v in list_of_vertexes {
        trace!("Checking {}, result {}",v,results[v]);
        if v < results.len() {
           if !is_first {
               print!(",");
            }
            print!("{}", results[v]);
            is_first = false;
        }
        else {
            error!("Dest Vertex {} is invalid",v);
        }
    }
    println!();

}



pub fn print_path_results(path_results: Vec<(usize,Vec<usize>,bool)> ) {

    let num_entries = path_results.len().clone();
    for (starting_vertex, path, has_cycle) in path_results {


        info!("Printing path results for {} items",num_entries);
        let mut first=true;
        let path_string : String = path.iter().map( |v| { if first { first=false; format!("{}",v) } else { format!(", {}",v) } } ).collect();

        print!("{} => path => {}",starting_vertex,path_string);
        if has_cycle {
            print!("... (has cycle)");
        }
        println!();
    }
}

fn main() {

    env_logger::init();

    let cmd_line = CommandArgs::parse();

    debug!("The Command Line, {:?}!",cmd_line);

    // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();


    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };


	let mut g = DirectedGraph::new();

    read_adjacency_multi(&mut file, &mut g );

//    let add_edge_fn = | s,d,w | g.add_edge(s,d,w) ;
////    read_adjacency_multi(&mut file, add_edge_fn);
    //g.print_vertexes();


    match &cmd_line.command {

        Some(Commands::Dijkstra { start, display_list, show_paths }) => {
            let mut d = Dijkstra::new(*start);

            for (id, _v) in g.vertex_iter() {
                d.initialize_vertex(id.clone());
            }
            d.calculate_shortest_paths(&g, *start);
            let results = d.get_shortest_path_distances();
            let list = match display_list {
                None => vec!(),
                Some(x) => x.clone(),
            };
            if *show_paths {
                d.print_paths(g.get_vertexes());
            }
            else {
                print_distance_result(results,list);
            }

        },
        Some(Commands::Bellman { start, display_list, show_paths }) => {
            let mut d = Bellman::new(g.vertex_count());

            info!("Staring Bellman");
            d.calculate_shortest_paths(&g, *start);
            let results = d.get_shortest_path_distances();
            let vertex_list = (1..=g.vertex_count()).collect();
            let path_results = d.get_shortest_paths(vertex_list);
            let list = match display_list {
                None => vec!(),
                Some(x) => x.clone(),
            };
            if d.has_negative_cycle() {
                println!("Negative cycle found...")
            }
            if *show_paths {
                print_path_results(path_results);
//                d.print_paths(g.get_vertexes());
            }
            else {
                print_distance_result(results,list);
            }

        },
        Some(Commands::Johnson { display_list, show_paths }) => {
            let vertex_list = g.get_vertexes();
            let mut j = Johnson::<'_>::new(&mut g);

            info!("Staring Johnson");
            j.calculate_shortest_paths();
            let list = match display_list {
                None => vec!(),
                Some(x) => x.clone(),
            };
            if j.has_negative_cycle() {
                println!("Negative cycle found...")
            }
            if *show_paths {
                j.print_paths(vertex_list);
            }
            else {
                j.print_result(list,true);
            }

        },
        Some(Commands::Verify {path}) => {
            let result = g.verify_path(path.to_vec());
            match result {
                Some(weight) => println!("Path is valid and has a weight of {}", weight),
                None =>  println!("Path is not valid"),
            }
        },
        Some(Commands::Print {..}) => {
            println!("Printing Graphs...");
        },
        None => {
            println!("No command given")

        },

    }
}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */
/*
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
*/
