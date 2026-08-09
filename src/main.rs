use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "data/tips.csv";
    let contents = fs::read_to_string(file_path)?;

    println!("Loaded {} bytes from {file_path}", contents.len());

    let header = contents.lines().next();

    match header {
        Some(line) => println!("Header: {line}"),
        None => println!("The CSV file is empty"),
    }

    Ok(())
}
