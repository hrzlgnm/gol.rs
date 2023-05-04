use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
struct Gol {
    grid: Vec<Vec<i8>>,
}

impl Display for Gol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.grid.iter().for_each(|r| {
            r.iter().for_each(|c| {
                match c {
                    1 => write!(f, "#").unwrap(),
                    _ => write!(f, " ").unwrap(),
                };
            });
            writeln!(f).unwrap();
        });
        Ok(())
    }
}

impl Gol {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![0; width]; height],
        }
    }

    pub fn cell_alive_at(&self, col: usize, row: usize) -> bool {
        self.grid[col as usize][row as usize] == 1
    }

    pub fn cell_at(&self, col: usize, row: usize) -> i8 {
        self.grid[col as usize][row as usize]
    }

    pub fn set_at(&mut self, col: usize, row: usize, cell: i8) {
        self.grid[col as usize][row as usize] = cell
    }

    pub fn alive_neighbours(&self, col: usize, row: usize) -> usize {
        let width = self.grid.len() as i32;
        let height = self.grid[0].len() as i32;
        let mut alive: usize = 0;
        for delta_row in 0..3 {
            for delta_col in 0..3 {
                let mut c = col as i32 + delta_col - 1;
                let mut r = row as i32 + delta_row - 1;
                if c >= width {
                    c %= width;
                }
                if r >= height {
                    r %= height;
                }
                if c < 0 {
                    c += width;
                }
                if r < 0 {
                    r += height;
                }
                alive += self.cell_alive_at(c as usize, r as usize) as usize;
            }
        }
        alive - self.cell_at(col, row) as usize
    }
}

fn next_board(board: Gol) -> Gol {
    let width = board.grid.len();
    let height = board.grid[0].len();

    let mut n = Gol::new(width, height);
    for row in 0..height {
        for col in 0..width {
            let nbors = board.alive_neighbours(col, row);
            match (board.cell_alive_at(col, row), nbors) {
                (true, 2) => n.set_at(col, row, 1),
                (true, 3) => n.set_at(col, row, 1),
                (false, 3) => n.set_at(col, row, 1),
                _ => {}
            }
        }
    }
    n
}

fn main() {
    let height: usize = 60;
    let width: usize = 60;
    let mut board = Gol::new(width, height);
    board.set_at(width / 2 - 1, height / 2 - 1, 0);
    board.set_at(width / 2 - 1, height / 2, 1);
    board.set_at(width / 2 - 1, height / 2 + 1, 1);
    board.set_at(width / 2, height / 2 - 1, 1);
    board.set_at(width / 2, height / 2, 1);
    board.set_at(width / 2, height / 2 + 1, 0);
    board.set_at(width / 2 + 1, height / 2 - 1, 0);
    board.set_at(width / 2 + 1, height / 2, 1);
    board.set_at(width / 2 + 1, height / 2 + 1, 0);

    for i in 1..10000 {
        print!("{esc}[1;1H{esc}[2J", esc = 27 as char);
        println!("G:{i}\n{board}");
        board = next_board(board);
        sleep(Duration::from_micros(1000 * 1000 / 30))
    }
}
