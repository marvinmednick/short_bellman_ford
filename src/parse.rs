use std::fs::File;
use std::io::{BufReader,BufRead};
use regex::Regex;
use log::{ info, /*error, */ debug, /*warn,*/ trace };


pub fn read_adjancency_with_weight<F: FnMut(u32,u32,i64)>
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
