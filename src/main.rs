use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// コマンドライン引数からファイルパスを返す
fn get_file_path() -> Result<String, io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("ファイルパスを引数として指定してください。");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "ファイルパスが不足しています"));
    }
    Ok(args[1].clone())
}

fn open_file_as_reader(path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn main() -> io::Result<()> {
    let path = get_file_path()?;
    let reader = open_file_as_reader(&path)?;

    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }

    Ok(())
}
