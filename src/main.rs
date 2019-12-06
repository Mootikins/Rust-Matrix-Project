/// Name: Sample Code
/// Author: Tyler Holinka, Matthew Krohn, Kendric Thompson, Jennifer Kulich
/// Class: CSC 461 - Programming Languages
/// Description: A basic project that gets vectors from a json file, and does linear algebra operations on them.
use operation::Operation;
use std::fs::File;

mod cli;
mod matrix;
mod operation;

/// Author: Tyler Holinka
/// Description: Function to get an Operation from a json file
/// Parameter input: the PathBuf representing the input file
/// Return: the Operation to run
fn get_opt(input: std::path::PathBuf) -> Operation {
    // make sure we have a valid in file, and open it
    let input = match File::open(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("need a valid file. {}", e);
            std::process::exit(1);
        }
    };
    match serde_json::from_reader(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("invalid json. {}", e);
            std::process::exit(1)
        }
    }
}

/// Author: Matthew Krohn
/// Description: The entry point for the program, runs the operation provided on the cli and exits
fn main() {
    let args = cli::process_args();

    let op = get_opt(args.input);

    op.do_operation_and_store();

    match args.out {
        None => println!("{}", op),
        Some(file) => {
            let out = File::create(file).unwrap();
            serde_json::to_writer_pretty(out, &op).expect("Unable to write to file");
        }
    }
}
