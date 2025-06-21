use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod simulator;
use simulator::{LifeGameSimulator};

/// コマンドライン引数からファイルパスを返す
fn get_file_path() -> Result<String, io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("ファイルパスを引数として指定してください。");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "ファイルパスが不足しています"));
    }
    Ok(args[1].clone())
}

// 引数で渡したファイルの中身を2次元配列として読み込む
pub fn read_grid_from_file(path: &str) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let row = line.chars()
                      .map(|c| if c == '#' { 1 } else { 0 })
                      .collect();
        grid.push(row);
    }

    Ok(grid)
}

fn main() -> io::Result<()> {
    let path = get_file_path()?;
    let grid = read_grid_from_file(&path)?;
    let mut simulator = LifeGameSimulator::new(grid).unwrap();
    simulator.start();
    Ok(())
}
