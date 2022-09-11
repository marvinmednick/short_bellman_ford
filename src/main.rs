use std::path::Path;
use std::fs::File;
use log::{  info , error, debug, /*warn, */trace };
use std::collections::BTreeMap;


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

fn print_distance_result(results: BTreeMap<usize,MinMax<i64>>, display_list: Vec<usize>) {


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
        trace!("Checking {}, result {}",v,results[&v]);
        if v < results.len() {
           if !is_first {
               print!(",");
            }
            print!("{}", results[&v]);
            is_first = false;
        }
        else {
            error!("Dest Vertex {} is invalid",v);
        }
    }
    println!();

}



pub fn print_path_results(path_results: BTreeMap<usize,(Vec<usize>,bool)> ) {

    let num_entries = path_results.len().clone();
    for (starting_vertex, (path, has_cycle)) in path_results {


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
            let vertex_list = (1..=g.vertex_count()).collect();
            let path_results = d.get_shortest_paths(vertex_list);
            let list = match display_list {
                None => vec!(),
                Some(x) => x.clone(),
            };
            if *show_paths {
                print_path_results(path_results);
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
            }
            else {
                print_distance_result(results,list);
            }

        },
        Some(Commands::Johnson { display_list, show_paths }) => {
            let vertex_list = g.get_vertex_ids();
            let mut j = Johnson::<'_>::new(&mut g);

            info!("Staring Johnson");
            j.calculate_shortest_paths();
            let list = match display_list {
                None => vec!(),
                Some(x) => x.clone(),
            };
            if j.has_negative_cycle() {
                info!("Negative cycle found...");
                if *show_paths {
                    println!("null");
                }
                else {
                    println!("NULL");
                }
            }
            else {
                if *show_paths {
                    j.print_paths(vertex_list);
                }
                else {
                    j.print_result(list,true);
                    for result in j.results_iter() {
                        println!("{:?}",result);
                    }
                }
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
            println!("Printing Graph...");
            g.print_graph();
        },
        None => {
            println!("No command given")

        },

    }
}


 #[cfg(test)]
mod tests {
    use super::*;



}
