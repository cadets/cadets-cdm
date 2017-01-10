#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

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
//         .subcommand(SubCommand::with_name("test")
//                     .about("controls testing features")
//                     .version("1.3")
//                     .author("Someone E. <someone_else@other.com>")
//                     .arg(Arg::with_name("debug")
//                          .short("d")
//                          .help("print debug information verbosely")))
        .get_matches();
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
