use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

fn main() -> Result<String, Error> {
    let filename = Path::new("doesnt_exist.txt");
    let mut file = File::open(&filename)?;

    let mut s = String::new();
    let size_read = file.read_to_string(&mut s)?;
    Ok(s)
}
