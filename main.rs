use std::collections::HashSet;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Dimensions {
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Cell {
    col: usize,
    row: usize,
}

type Cells = HashSet<Cell>;

#[derive(Clone)]
struct Grid {
    dim: Dimensions,
    alive_cells: Cells,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dim.height {
            for col in 0..self.dim.width {
                if self.cell_is_alive_at(col, row) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Neighbours {
    alive: Cells,
    dead: Cells,
}

impl Grid {
    pub fn new(dim: Dimensions, alive_cells: HashSet<Cell>) -> Self {
        Self {
            dim,
            alive_cells,
        }
    }

    pub fn cell_is_alive_at(&self, col: usize, row: usize) -> bool {
        self.alive_cells.contains(&Cell { col, row })
    }

    pub fn add_cell(&mut self, col: usize, row: usize) {
        self.alive_cells.insert(Cell { col, row });
    }

    pub fn remove_cell(&mut self, col: usize, row: usize) {
        self.alive_cells.remove(&Cell { col, row });
    }

    pub fn get_neighbours(&self, col: usize, row: usize) -> Neighbours {
        const OFFSETS : [(isize, isize); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0),           (1, 0),
            (-1, 1),  (0, 1),  (1, 1),
        ];
        let possible_neighbours = OFFSETS.iter().map(|(x, y)| (col as isize + x, row as isize + y));
        let mut alive = HashSet::new();
        let mut dead = HashSet::new();
        for (x, y) in possible_neighbours {
            if x < 0 || y < 0 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if x >= self.dim.width || y >= self.dim.height {
                continue;
            }
            if self.cell_is_alive_at(x, y) {
                alive.insert(Cell { col: x, row: y });
            } else {
                dead.insert(Cell { col: x, row: y });
            }
        }
        Neighbours { alive, dead }
    }
}

fn update_grid(grid: Grid) -> Grid {
    let mut new_grid = grid.clone();
    for cell in grid.alive_cells.iter() {
        let nbors = grid.get_neighbours(cell.col, cell.row);
        match nbors.alive.len() {
            2 | 3 => {}
            _ => {
                new_grid.remove_cell(cell.col, cell.row);
            }
        }
        for cell in nbors.dead {
            let nbors = grid.get_neighbours(cell.col, cell.row);
            if nbors.alive.len() == 3 {
                new_grid.add_cell(cell.col, cell.row);
            }
        }
    }
    new_grid
}

fn gosper_glider() -> Grid {
    let mut grid = Grid::new(
        Dimensions {
            width: 60,
            height: 60,
        },
        HashSet::new(),
    );
    grid.add_cell(1, 5);
    grid.add_cell(2, 5);
    grid.add_cell(1, 6);
    grid.add_cell(2, 6);
    grid.add_cell(11, 5);
    grid.add_cell(11, 6);
    grid.add_cell(11, 7);
    grid.add_cell(12, 4);
    grid.add_cell(12, 8);
    grid.add_cell(13, 3);
    grid.add_cell(13, 9);
    grid.add_cell(14, 3);
    grid.add_cell(14, 9);
    grid.add_cell(15, 6);
    grid.add_cell(16, 4);
    grid.add_cell(16, 8);
    grid.add_cell(17, 5);
    grid.add_cell(17, 6);
    grid.add_cell(17, 7);
    grid.add_cell(18, 6);
    grid.add_cell(21, 3);
    grid.add_cell(21, 4);
    grid.add_cell(21, 5);
    grid.add_cell(22, 3);
    grid.add_cell(22, 4);
    grid.add_cell(22, 5);
    grid.add_cell(23, 2);
    grid.add_cell(23, 6);
    grid.add_cell(25, 1);
    grid.add_cell(25, 2);
    grid.add_cell(25, 6);
    grid.add_cell(25, 7);
    grid.add_cell(35, 3);
    grid.add_cell(35, 4);
    grid.add_cell(36, 3);
    grid.add_cell(36, 4);
    grid
}


fn main() {
    let mut grid = gosper_glider();

    for i in 1..1000 {
        print!("{esc}[1;1H{esc}[2J", esc = 27 as char);
        println!("G:{i}\n{board}", board = grid, i = i);
        grid = update_grid(grid);
        sleep(Duration::from_micros(1000 * 1000 / 30))
    }
}
