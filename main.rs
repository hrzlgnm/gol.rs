use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Dead => write!(f, " "),
            Cell::Alive => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
struct Board {
    width: i32,
    height: i32,
    board: Vec<Cell>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.cell_at(col, row))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            board: vec![Cell::Dead; (width * height) as usize],
        }
    }

    pub fn cell_alive_at(&self, col: i32, row: i32) -> bool {
        self.board[(row * self.width + col) as usize] == Cell::Alive
    }

    pub fn cell_at(&self, col: i32, row: i32) -> Cell {
        self.board[(row * self.width + col) as usize]
    }

    pub fn set_at(&mut self, col: i32, row: i32, cell: Cell) {
        self.board[(row * self.width + col) as usize] = cell;
    }

    pub fn alive_neighbours(&self, col: i32, row: i32) -> usize {
        let mut alive: usize = 0;
        for delta_row in 0..3 {
            for delta_col in 0..3 {
                if delta_row != 1 || delta_col != 1 {
                    let mut c = col + delta_col - 1;
                    let mut r = row + delta_row - 1;
                    if c >= self.width {
                        c %= self.width;
                    }
                    if r >= self.height {
                       r %= self.height;
                    }
                    if c < 0 {
                        c += self.width;
                    }
                    if r < 0 {
                        r += self.height;
                    }
                    if self.cell_alive_at(c, r) {
                        alive += 1;
                    }
                }
            }
        }
        alive
    }
}

fn next_board(board: Board) -> Board {
    let mut n = Board::new(board.width, board.height);
    for row in 0..board.height {
        for col in 0..board.width {
            let nbors = board.alive_neighbours(col, row);
            match (board.cell_alive_at(col, row), nbors) {
                (true, 2) => n.set_at(col, row, Cell::Alive),
                (true, 3) => n.set_at(col, row, Cell::Alive),
                (false, 3) => n.set_at(col, row, Cell::Alive),
                _ => {}
            }
        }
    }
    n
}

fn main() {
    let height: i32 = 60;
    let width: i32 = 60;
    let mut board = Board::new(width, height);
    board.set_at(width / 2 - 1, height / 2 - 1, Cell::Dead);
    board.set_at(width / 2 - 1, height / 2,  Cell::Alive);
    board.set_at(width / 2 - 1, height / 2 + 1, Cell::Alive);
    board.set_at(width / 2, height / 2 - 1, Cell::Alive);
    board.set_at(width / 2, height / 2, Cell::Alive);
    board.set_at(width / 2, height / 2 + 1, Cell::Dead);
    board.set_at(width / 2 + 1, height / 2 - 1, Cell::Dead);
    board.set_at(width / 2 + 1, height / 2, Cell::Alive);
    board.set_at(width / 2 + 1, height / 2 + 1, Cell::Dead);

    for i in 1..10000 {
        print!("{esc}[1;1H{esc}[2J", esc = 27 as char);
        println!("G:{i}\n{board}");
        board = next_board(board);
        sleep(Duration::from_micros(1000 * 1000 / 30))
    }
}
