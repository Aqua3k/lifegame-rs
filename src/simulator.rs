use std::cmp::max;

const PADDING_SIZE: usize = 10;

struct LifeGameSimulator {
    grid: Vec<Vec<CellStatus>>,
}

#[derive(Clone)]
enum CellStatus {
    DEAD,
    ALIVE,
}

impl LifeGameSimulator {
    fn new(status: Vec<Vec<u8>>) -> Self{
        // 実際にシミュレートする配列サイズを決定
        let height = status.len();
        let mut max_width: usize = 0;
        for array in status.clone() {
            max_width = max(max_width, array.len());
        }
        let simulate_width = max_width + (2 * PADDING_SIZE);
        let simulate_height = height + (2 * PADDING_SIZE);

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
            assert!(array.len() == simulate_width);
        }

        // 行方向 後半のパディングを入れる
        for i in 0..PADDING_SIZE {
            let mut array: Vec<CellStatus> = vec![CellStatus::DEAD; simulate_width];
            grid.push(array);
        }

        Self { grid: grid }
    }
}
