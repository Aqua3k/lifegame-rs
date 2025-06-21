use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod simulator;

/// コマンドライン引数からファイルパスを返す
fn get_file_path() -> Result<String, io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("ファイルパスを引数として指定してください。");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "ファイルパスが不足しています"));
    }
    Ok(args[1].clone())
}

/// 引数のファイルの内容を出力する
fn print_file(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    println!("#----------------#");
        for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }
    println!("#----------------#");

    Ok(())
}

fn main() -> io::Result<()> {
    let path = get_file_path()?;
    print_file(&path)?;
    Ok(())
}
