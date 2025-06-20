use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn open_file_as_reader(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn main() -> io::Result<()> {
    let path = "input.txt";
    let reader = open_file_as_reader(path)?;

    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }

    Ok(())
}
