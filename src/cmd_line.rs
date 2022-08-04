extern crate clap;
//use log::{ info , error /* ,debug, warn,trace */ };

//use clap::{Arg, Command,arg, Parser, Subcommand};
use clap::{Parser, Subcommand};

#[derive(Debug)]
pub enum InputFileFormat {
    AdjacentSingleEntryPerLine,
    AdjacentMultiEntryPerLine,
}

#[derive(Parser, Debug)]
#[clap(name = "short")]
#[clap(author = "Marvin Mednick")]
#[clap(version = "1.0")]
#[clap(about = "Various Shortest path utilities ", long_about = "Supports Dijkstra and will support bellman-ford and others.. ")]
pub struct CommandArgs  {
    #[clap(value_parser)]
    pub filename: String,
    //pub input_format: InputFileFormat,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}


#[derive(Subcommand, Debug)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long, action)]
        list: bool,
    },
    /// Executes Dijkstra shortest path on graph
    Dijkstra {
        #[clap(value_parser)]
        start: u32,
    }
}

/*
#[derive(Debug)]
pub struct CommandArgs  {
    pub filename: String,
    pub dijkstra: bool,
    pub dijkstra_start: u32,
    pub input_format: InputFileFormat,
}

impl CommandArgs  {
    pub fn new() -> Self {
        // basic app information
        let app = Command::new("short")
            .version("1.0")
            .about("Shortest Path Cacluations")
            .author("Marvin Mednick");

        // Define the name command line option
        let filename_option = Arg::new("file")
            .takes_value(true)
            .help("Input file name")
            .required(true);

        let format_option = Arg::new("format")
            .short('f')
            .long("format")
            .takes_value(true)
            .required(true)
            .value_name("FORMAT")
            .help("format of input file");

        let dijkstra_option = Arg::new("dijkstra")
            .long("dijkstra")
            .takes_value(true)
            .help("compute shortest path via dijkstra");

        // now add in the argument we want to parse
        let app = app.arg(filename_option)
                     .arg(dijkstra_option);

        // extract the matches
        let matches = app.get_matches();

        // Extract the actual name
        let filename = matches.value_of("file")
            .expect("Filename can't be None, we said it was required");

        let run_dijkstra = matches.is_present("dijkstra");
        let num_str = matches.value_of("dijkstra");

        let starting_vertex = match num_str {
            None => { println!("Start is None..."); 0},
            Some(s) => {
                match s.parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => {println!("That's not a number! {}", s); 0},
                }
            }
        };

        let format_str = matches.value_of("format_option")
            .expect("Format can't be None, we said it was required");

        let file_format = match format_str {
            "multi" => InputFileFormat::AdjacentMultiEntryPerLine,
            "single" => InputFileFormat::AdjacentSingleEntryPerLine,
            &_ => {error!("Unnown input format ");  InputFileFormat::AdjacentSingleEntryPerLine },
        };

        info!("clap args: {}",filename );

        CommandArgs { filename: filename.to_string(),
                      dijkstra: run_dijkstra,
                      dijkstra_start : starting_vertex,
                      input_format: file_format,
        }
    }   
}
*/
