use std::fs::File;
use std::io::{BufReader,BufRead};
use regex::Regex;
use log::{ info, /*error, */ debug, /*warn,*/ trace };


pub fn read_adjacency_single<F: FnMut(u32,u32,i64)>
    ( file: & mut File, mut add_edge: F ) {

    //open the file
    let mut reader = BufReader::new(file);

    // read the first line
    let mut line = String::new();
    let _len = reader.read_line(&mut line).unwrap();
    info!("First Input Line is \'{}\'",line);

    // parse the first line which includes number of vertexes and number of edges
    let first_line_regex = Regex::new(r"\s*(?P<num_vertex>\d+)\s+(?P<num_edges>\d+)\s+.*$").unwrap();
    let caps = first_line_regex.captures(&line).unwrap();
    let _num_vertex = caps["num_vertex"].parse::<u32>().unwrap(); 
    let _num_edges = caps["num_edges"].parse::<u32>().unwrap(); 

	let mut count = 0;
    for line in reader.lines() {
		count += 1;	
		let line_data = line.unwrap();
        debug!("Processing {} {}",count, line_data);
        if count % 50 == 0 {
            info!("Proccesed {}", count);
        }
        let line_regex = Regex::new(r"\s*(?P<source>\d+)\s+(?P<dest>\d+)\s+(?P<weight>-?\d+)*$").unwrap();
        trace!("Line is {}",line_data);
        let caps = line_regex.captures(&line_data).unwrap();
        trace!("Caps is {:#?}",caps);
        let source = caps["source"].parse::<u32>().unwrap(); 
        let dest = caps["dest"].parse::<u32>().unwrap(); 
        let weight = caps["weight"].parse::<i64>().unwrap(); 
        add_edge(source,dest, weight);
    }

}


// 
// Format is 1 line per vertex with a tuple consistenting of destination vertex and weight
// e.g.    
// 1   2,8   3,6
// 2   1,8  3, 4
// 3   1,6, 2, 4
pub fn read_adjacency_multi<F: FnMut(u32,u32,i64)>
    ( file: & mut File, mut add_edge: F ) {

    //open the file
    let reader = BufReader::new(file);

	let mut _count = 0;
    for line in reader.lines() {
		_count += 1;	
		let line_data = line.unwrap();

        // split the line into the vertex and the list of adjacent vertexes/weight pairs
        let re_vertex = Regex::new(r"\s*(?P<vertex>\d+)\s+(?P<adjacent_list>.*$)").unwrap();
        // adjacent vertexes are in the format vertex,weight   - and regex below allows for
        // whitespace
        let caps = re_vertex.captures(&line_data).unwrap();
        let text1 = caps.get(1).map_or("", |m| m.as_str());
        let vertex = text1.parse::<u32>().unwrap();
        debug!("Reading connectsion for vertex {}",vertex);

        let re_adjacent = Regex::new(r"\s*(?P<vertex>\d+)\s*(,|\s)\s*(?P<weight>\d*)").unwrap();
        let text2 = caps.get(2).map_or("", |m| m.as_str());
        trace!("Adjacency info: {}",text2);


        let mut _count =0;
        for caps in re_adjacent.captures_iter(text2) {
            let dest_vertex = caps["vertex"].parse::<u32>().unwrap(); 
            let weight = caps["weight"].parse::<i64>().unwrap(); 
            debug!("Adding connection from {} to {} with weight {}",vertex,dest_vertex,weight);
			let _num_edges = add_edge(vertex,dest_vertex,weight);
            _count += 1;

        }
    }
}
