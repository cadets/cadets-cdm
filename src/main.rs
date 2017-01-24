#[macro_use]
extern crate clap;
use clap::{Arg, App};
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

// This is the main function
fn main() {
    let matches = App::new("CADETS CDM Translator")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Translate the CADETS trace format to CDM")
        .arg(Arg::with_name("inputs-trace")
             .short("i")
             .long("input-trace")
             .value_name("file")
             .help("sets the input file(s)")
             .multiple(true)
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("v")
             .short("v")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .get_matches();

    // Loop through all the files given
    let files: Vec<_> = matches.values_of("inputs-trace").unwrap().collect();
    for arg in files {
        let path = Path::new(arg);
        println!("Translating file: {:?}", path.display());
        handle_file(path);
    }


}

// Print the beginning of each file
fn handle_file(filepath : &Path)
{
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&filepath) {
        Err(err) => panic!("couldn't open {}: {}", filepath.display(),
                                                   err.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("couldn't read {}: {}", filepath.display(),
                           err.description()),
        Ok(_) => {
            s.truncate(20); // let's not print the entire file
            println!("{} starts with:\n{}...", filepath.display(), s)
        }
    }
    println!("Done translating {}", filepath.display())
}
