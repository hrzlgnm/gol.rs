#[derive(Debug,Clone,Copy,PartialEq)]
enum Cell {
    DEAD,
    ALIVE,
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    board: Vec<Cell>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            board: vec![Cell::DEAD; width*height],
        }
    }

    pub fn cell_at(&self, col: usize, row: usize) -> Box<Cell> {
        return Box::new(self.board[row * self.width + col]);
    }

    pub fn set_at(&mut self, col: usize, row: usize, cell: Cell) {
        self.board[row * self.width + col] = cell;
    }

    pub fn alive_neighbours(&self, col: usize, row: usize) -> usize {
        let mut alive: usize = 0;
        for delta_col in 0..2  {
            for delta_row in 0..2 {
                if delta_row != 1 || delta_col != 1 {
                    let c = col.wrapping_add(delta_col).wrapping_sub(1) % self.width;
                    let r = row.wrapping_add(delta_row).wrapping_sub(1) % self.height;
                    let cel = self.cell_at(r, c);
                    if *cel == Cell::ALIVE {
                        alive+=1;
                    }
                }
            }                    
        }
        alive
    }
}

fn next_board(board: Board) -> Board {
    let mut n = Board::new(board.width, board.height);
    for col in 0..board.width {
        for row in 0..board.height {
            let nbors = board.alive_neighbours(col, row);
            match Some(*board.cell_at(col, row)) {
                Some(Cell::ALIVE) if nbors == 2 || nbors == 3 => {
                    n.set_at(col, row, Cell::ALIVE)
                },
                Some(Cell::DEAD) if nbors == 3 => {
                    n.set_at(col, row, Cell::DEAD)
                }
                _ => {

                }
            }
        }
    }
    n
}

fn main() {
   let mut board = Board::new(10, 10);
   board.set_at(0, 0, Cell::ALIVE);
   let b = next_board(board);
   println!("{:?}", b);
}

