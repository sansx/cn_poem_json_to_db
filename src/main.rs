use serde_json::Value;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");
    let path = "./guwen/test.json";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines();
    lines.next().unwrap()?;
    let mut test_target = lines.next().unwrap()?;
    let v: Value = serde_json::from_str(test_target.as_str())?;

    println!("{:?}", v);

    Ok(())
}
