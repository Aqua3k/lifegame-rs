use std::{cell::Cell, cmp::max};
use std::thread;
use std::time::Duration;

const PADDING_SIZE: usize = 10; // シミュレータ内で拡張するgridのサイズ
const UPDATE_RATE: u64 = 100; // 描画更新の間隔(ミリ秒単位)

const MAX_SIZE: usize = 100; // シミュレート可能なサイズの上限

const ADJACENT_DELTAS: [(i8, i8); 8] = [
    (-1, -1), (-1,  0), (-1,  1),
    ( 0, -1),          ( 0,  1),
    ( 1, -1), ( 1,  0), ( 1,  1),
];

struct LifeGameSimulator {
    grid: Vec<Vec<CellStatus>>,
    simulate_width: usize,
    simulate_height: usize,
}

#[derive(Clone, PartialEq)]
enum CellStatus {
    DEAD,
    ALIVE,
}

impl LifeGameSimulator {
    pub fn new(status: Vec<Vec<u8>>) -> Result<Self, String> {
        // 実際にシミュレートする配列サイズを決定
        let height = status.len();
        let mut max_width: usize = 0;
        for array in status.clone() {
            max_width = max(max_width, array.len());
        }
        let simulate_width = max_width + (2 * PADDING_SIZE);
        let simulate_height = height + (2 * PADDING_SIZE);

        if MAX_SIZE < simulate_width || MAX_SIZE < simulate_height {
            return Err("サイズ上限オーバー".to_string());
        }

        // 行方向 前半のパディングを入れる
        let mut grid: Vec<Vec<CellStatus>> = Vec::new();
        for i in 0..PADDING_SIZE {
            let mut array: Vec<CellStatus> = vec![CellStatus::DEAD; simulate_width];
            grid.push(array);
        }

        // 行方向 本体データを作る
        for i in 0..height {
            let mut array: Vec<CellStatus> = Vec::new();
            // 列方向 前半のパディングを入れる
            for j in 0..PADDING_SIZE {
                array.push(CellStatus::DEAD);
            }

            // 列方向 本体データを作る
            for j in 0..max_width {
                let status = if status[i][j] == 0 {
                    CellStatus::ALIVE
                } else {
                    CellStatus::DEAD
                };
                array.push(status);
            }

            // 列方向 後半のパディングを入れる
            for j in 0..PADDING_SIZE {
                array.push(CellStatus::DEAD);
            }

            // サイズチェック
            assert!(array.len() == simulate_width);
        }

        // 行方向 後半のパディングを入れる
        for i in 0..PADDING_SIZE {
            let mut array: Vec<CellStatus> = vec![CellStatus::DEAD; simulate_width];
            grid.push(array);
        }

        // サイズチェック
        assert!(grid.len() == simulate_height);

        Ok(Self {
            grid: grid,
            simulate_width: simulate_width,
            simulate_height: simulate_height,
        })
    }

    /// セル(x, y)の状態取得
    fn get_cell_status(&self, x: i8, y: i8) -> CellStatus {
        if x < 0 || self.simulate_height <= x as usize {
            return CellStatus::DEAD;
        }
        if y < 0 || self.simulate_width <= y as usize {
            return CellStatus::DEAD;
        }
        self.grid[x as usize][y as usize].clone()
    }

    /// セル(x, y)の次ターンの状態を決定
    fn get_next_status(&self, x: i8, y: i8) -> CellStatus {
        // 周囲で生きているセルの数を数える
        let mut living_cell_count = 0;
        for &(dx, dy) in ADJACENT_DELTAS.iter() {
            if self.get_cell_status(x + dx, y + dy) == CellStatus::ALIVE {
                living_cell_count += 1;
            }
        }

        // セル状態から次のターンの状態を決定
        let status = self.get_cell_status(x, y);
        if status == CellStatus::ALIVE {
            if living_cell_count <= 1 {
                // 過疎
                CellStatus::DEAD
            } else if living_cell_count <= 3 {
                // 適正
                CellStatus::ALIVE
            } else {
                // 過密
                CellStatus::DEAD
            }
        } else {
            if living_cell_count == 3 {
                // 誕生
                CellStatus::ALIVE
            } else {
                CellStatus::DEAD
            }
        } 
    }

    /// 1ターン進める
    fn next(&mut self) {
        let mut next_grid: Vec<Vec<CellStatus>> = Vec::new();
        for i in 0..self.simulate_height {
            let mut array: Vec<CellStatus> = Vec::new();
            for j in 0..self.simulate_width {
                let next_cell = self.get_next_status(i as i8, j as i8);
                array.push(next_cell);
            }
            next_grid.push(array);
        }
        self.grid = next_grid;
    }

    /// シミュレート中のメソッドをコンソールに描画する
    fn display(&self) {
        // TODO とりあえずprintするだけの実装にする
        println!("#------------------#");
        for array in self.grid.clone() {
            for status in array {
                let string = if status == CellStatus::ALIVE {
                    "#"
                } else {
                    " "
                };
                print!("{}", string);
            }
            println!("");
        }
        println!("#------------------#");
    }

    /// シミュレートを開始する
    pub fn start(&mut self) {
        let simulate_turn = 50; // TODO
        self.display();
        thread::sleep(Duration::from_millis(UPDATE_RATE));
        for i in 0..simulate_turn {
            self.next();
            self.display();
            thread::sleep(Duration::from_millis(UPDATE_RATE));
        }
    }
}
