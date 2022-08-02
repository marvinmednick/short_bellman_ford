extern crate clap; use log::{ info /*, error, debug, warn,trace */ };

use clap::{Arg, Command};

#[derive(Debug)]
pub struct CommandArgs  {
    pub filename: String,
}

impl CommandArgs  {
    pub fn new() -> Self {
        // basic app information
        let app = Command::new("cluster")
            .version("1.0")
            .about("Determines clustering")
            .author("Marvin Mednick");

        // Define the name command line option
        let filename_option = Arg::new("file")
            .takes_value(true)
            .help("Input file name")
            .required(true);


        // now add in the argument we want to parse
        let app = app.arg(filename_option);

        // extract the matches
        let matches = app.get_matches();

        // Extract the actual name
        let filename = matches.value_of("file")
            .expect("Filename can't be None, we said it was required");


        info!("clap args: {}",filename );

        CommandArgs { filename: filename.to_string() }
    }   
}
