use std::path::PathBuf;

/// Author: Tyler Holinka
/// Description: The "internal" representation of the command line arguments
#[derive(PartialEq, Eq)]
pub struct Arguments {
    pub debug: bool,
    pub input: PathBuf,
    pub out: Option<PathBuf>,
}

impl std::fmt::Debug for Arguments {
    /// Author: Tyler Holinka
    /// Description: Implements fmt::Debug, so "{:?}" can be used to print the Arguments
    /// Parameter self: reference to this instance of Arguments
    /// Parameter f: reference to a formatter stream
    /// Return: The result of the write to the formatter stream
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Debug: {}", self.debug)?;
        writeln!(f, "Input: {:?}", self.input)?;

        if let Some(out) = self.out_as_ref() {
            // don't output "Some(PathBuf)", instead output "PathBuf"
            writeln!(f, "Out: {:?}", out)
        } else {
            f.write_str("Out: None")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// Author: Tyler Holinka
    /// Description: Tests the fmt::Debug trait of the Arguments struct
    #[test]
    fn test_fmt_debug() {
        // only input
        let args = Arguments {
            debug: false,
            input: PathBuf::from_str("test-file.json").unwrap(),
            out: None,
        };

        let fmt = format!("{:?}", args);
        assert_eq!(fmt, "Debug: false\nInput: \"test-file.json\"\nOut: None\n");

        // debug + out as well
        let args = Arguments {
            debug: true,
            input: PathBuf::from_str("test-file.json").unwrap(),
            out: Some(PathBuf::from_str("test-out.json").unwrap()),
        };

        let fmt = format!("{:?}", args);
        assert_eq!(
            fmt,
            "Debug: true\nInput: \"test-file.json\"\nOut: \"test-out.json\"\n"
        );
    }

    /// Author: Tyler Holinka
    /// Description: Tests the cmp::PartialEq trait of the Arguments struct
    #[test]
    fn test_cmd_partialeq() {
        let args = Arguments {
            debug: true,
            input: PathBuf::from_str("test-file.json").unwrap(),
            out: Some(PathBuf::from_str("test-out.json").unwrap()),
        };

        // equal
        assert_eq!(
            args,
            Arguments {
                debug: true,
                input: PathBuf::from_str("test-file.json").unwrap(),
                out: Some(PathBuf::from_str("test-out.json").unwrap()),
            }
        );

        // debug different
        assert_ne!(
            args,
            Arguments {
                debug: false,
                input: PathBuf::from_str("test-file.json").unwrap(),
                out: Some(PathBuf::from_str("test-out.json").unwrap()),
            }
        );

        // input different
        assert_ne!(
            args,
            Arguments {
                debug: true,
                input: PathBuf::from_str("different-test-file.json").unwrap(),
                out: Some(PathBuf::from_str("test-out.json").unwrap()),
            }
        );

        // out different
        assert_ne!(
            args,
            Arguments {
                debug: true,
                input: PathBuf::from_str("test-file.json").unwrap(),
                out: Some(PathBuf::from_str("different-test-out.json").unwrap()),
            }
        )
    }
}
