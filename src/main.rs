/**************************************
* Author: Eric Hansson
* Date:
* Purpose:
**************************************/
use std::error::Error;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<_> = env::args().collect();
    let path = Path::new(&args[2]);

    let file = fs::read_to_string(path)?;

    println!("{:?}", file.as_bytes());

    Ok(())
}
