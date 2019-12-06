use std::path::PathBuf;

/// Author: Tyler Holinka
/// Description: The "internal" representation of the command line arguments
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

        if self.out == None {
            writeln!(f, "Out: {:?}", self.out)
        } else {
            // don't output "Some(PathBuf)", instead output "PathBuf"
            writeln!(f, "Out: {:?}", self.out.as_ref().unwrap())
        }
    }
}

impl std::cmp::PartialEq for Arguments {
    /// Author: Tyler Holinka
    /// Description: Implements PartialEq so Arguments, which allows comparing Arguments
    /// Parameter self: reference to this instance of Arguments
    /// Parameter other: reference to another instance of Arguments
    /// Return: true if the two references are equal, false otherwise
    fn eq(&self, other: &Self) -> bool {
        if self.debug != other.debug {
            return false;
        } else if self.input != other.input {
            return false;
        } else if self.out != other.out {
            return false;
        }
        return true;
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
