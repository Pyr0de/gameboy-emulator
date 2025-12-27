mod registers;
mod memory_mapping;

use std::{fs::File, io::{Error, Read}};

fn main() -> Result<(), Error> {
    let mut file = File::open("")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(())
}
