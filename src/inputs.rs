use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Opens a file and returns a vector containing the representation of each line of the file
///
/// # Errors
///
/// This function will return an error if:
///
/// * The path provided does not exist or is inaccessible
/// * The data can not be parsed into type `T`
pub fn get_input<T>(path: &str) -> Result<Vec<T>, io::Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = File::open(path)?;

    let file_reader = BufReader::new(f);
    let mut vals: Vec<T> = vec![];

    for line in file_reader.lines() {
        let Ok(val) = line?.trim().parse() else {
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could parse data for type specified!",
                ));
            }
        };

        vals.push(val);
    }

    Ok(vals)
}

/// Opens a file and returns a vector containing the representation of each line of the file,
/// also separating based on a delimiter string
///
/// # Errors
///
/// This function will return an error if:
///
/// * The path provided does not exist or is inaccessible
/// * The data can not be parsed into type `T`
pub fn get_input_delim<T>(path: &str, delim: &str) -> Result<Vec<T>, io::Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = File::open(path)?;
    let file_reader = BufReader::new(f);
    let mut vals: Vec<T> = vec![];

    for line in file_reader.lines() {
        for val_str in line?.split(delim) {
            let Ok(val) = val_str.trim().parse() else {
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Could parse data for type specified!",
                    ));
                }
            };

            vals.push(val);
        }
    }

    Ok(vals)
}
