use arguments::Arguments;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

mod arguments;

/// Author: Tyler Holinka
/// Description: Tells the StructOpt crate what command line arguments we are looking for
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Linear Algebra",
    about = "Sample Linear Algebra Operations.",
    no_version
)]
pub struct Opt {
    // note: triple slash is used by StructOpt as the "description" by default
    /// activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// input json file
    #[structopt(parse(from_str), short, long, required = true)]
    in_file: PathBuf,

    /// output json file
    #[structopt(parse(try_from_str), short, long, required = false, default_value = "")]
    out_file: String,
}

/// Author: Tyler Holinka
/// Description: converts arguments from StructOpt form to an internal (Arguments) representation
/// Argument opt: The Opt provided by StructOpt
/// Return: Our internal representation of arguments
fn process(opt: Opt) -> Arguments {
    // convert the outfile to an Option
    let out: Option<PathBuf> = match opt.out_file.as_ref() {
        "" => None,
        _ => PathBuf::from_str(&opt.out_file).ok(), 
    };

    Arguments {
        debug: opt.debug,
        input: opt.in_file,
        out,
    }
}

/// Author: Tyler Holinka
/// Description: Wrapper for process to decouple getting arguments from processing them
///              to make it easier to test
/// Return: Our internal representation of arguments
pub fn process_args() -> Arguments {
    process(Opt::from_args())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Author: Tyler Holinka
    /// Description: test only having no input file on the command line
    #[test]
    fn opt_test_no_input() {
        let opt = Opt::from_iter_safe(&["test"]);

        match opt {
            Ok(_) => panic!("no input file should panic, not succeed"),
            Err(_) => (),
        }
    }

    /// Author: Tyler Holinka
    /// Description: test only having the input file on the command line
    #[test]
    fn process_test_only_input() {
        let file = "test-input.json";
        let expected = Arguments {
            debug: false,
            input: PathBuf::from_str(file).unwrap(),
            out: None,
        };

        let opt = Opt::from_iter(&["test", "-i", file]);

        let args = process(opt);

        assert_eq!(args, expected)
    }

    /// Author: Tyler Holinka
    /// Description: test an out file
    #[test]
    fn process_test_out() {
        let input = "test-input.json";
        let out = "test-out.json";
        let expected = Arguments {
            debug: false,
            input: PathBuf::from_str(input).unwrap(),
            out: PathBuf::from_str(out).ok(),
        };

        let opt = Opt::from_iter(&["test", "-i", input, "-o", out]);

        let args = process(opt);

        assert_eq!(args, expected)
    }

    /// Author: Tyler Holinka
    /// Description: test an out file
    #[test]
    fn process_test_debug() {
        let input = "test-input.json";

        let expected = Arguments {
            debug: true,
            input: PathBuf::from_str(input).unwrap(),
            out: None,
        };

        let opt = Opt::from_iter(&["test", "-i", input, "-d"]);

        let args = process(opt);

        assert_eq!(args, expected)
    }
}
