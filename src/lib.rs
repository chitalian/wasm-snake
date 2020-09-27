mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;
use std::collections::VecDeque;
use rand;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
    Food = 2,
}
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cord {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    direction: Direction,
    snake: VecDeque<Cord>,
    is_gameover: bool,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn is_gameover(&self) -> bool {
        self.is_gameover
    }
    pub fn change_direction(&mut self, direction: Direction) {
        
        self.direction = match direction {
            Direction::Up if self.direction != Direction::Down => direction,
            Direction::Right if self.direction != Direction::Left => direction,
            Direction::Down if self.direction != Direction::Up => direction,
            Direction::Left if self.direction != Direction::Right => direction,
            _ => self.direction,
        };
    }
    pub fn tick(&mut self) {
        // let mut next = self.cells.clone();
        let head = self.snake.front().unwrap();
        let idx = self.get_index(head.x, head.y); 
        self.cells[idx] = Cell::Dead;
        let new_head = match self.direction {
            Direction::Up if head.x == 0 => (self.height - 1, head.y),
            Direction::Up => (head.x - 1, head.y),
            Direction::Right => (head.x, (head.y + 1) % self.width),
            Direction::Down => ((head.x + 1) % self.height, head.y),
            Direction::Left if head.y == 0 => (head.x, self.width - 1),
            Direction::Left => (head.x, (head.y - 1)),
        };
        self.snake.push_front(Cord{x: new_head.0, y: new_head.1});
        match self.cells[self.get_index(new_head.0, new_head.1)] {
            Cell::Food => { self.place_food(); },
            Cell::Dead => {
                let released = self.snake.pop_back().unwrap();
                let idx = self.get_index(released.x, released.y); 
                self.cells[idx] = Cell::Dead;
            },
            Cell::Alive => {
                self.is_gameover = true;
            }
        };



        
        // let idx = self.get_index(head.x, head.y); 
        
        for cord in self.snake.iter() {
            let idx = self.get_index(cord.x, cord.y); 
            self.cells[idx] = Cell::Alive;
        }
        // self.cells[idx] = Cell::Alive;

        // self.cells = next;
    }

    fn cell_in_snake(&self, cell: usize) -> bool {
        for sn_cell in self.snake.iter() {
            if cell == self.get_index(sn_cell.x, sn_cell.y) {
                return true;
            }
        }
        return false;
    }

    fn place_food(&mut self) {
        let mut x = rand::random::<usize>() % self.cells.len();
        while self.cell_in_snake(x) {
            x = rand::random::<usize>() % self.cells.len();
        }
        self.cells[x] = Cell::Food;
    }

    pub fn new() -> Universe {
        let width = 50;
        let height = 50;

        let cells: Vec<Cell> = (0..width * height)
            .map(|_| {Cell::Dead})
            .collect();
        let direction = Direction::Right;
        

        let mut snake = VecDeque::new();
        snake.push_back(Cord{x:5, y:5});
        // snake.push_back(Cord{x:5, y:6});
        // snake.push_back(Cord{x:5, y:7});
        // snake.push_back(Cord{x:5, y:8});
        // snake.push_back(Cord{x:5, y:9});
        // snake.push_back(Cord{x:5, y:10});
        // snake.push_back(Cord{x:5, y:11});

        // let idx = 60;
        // cells[idx] = Cell::Alive;
        let mut u = Universe {
            width,
            height,
            cells,
            direction,
            snake,
            is_gameover: false,
        };
        u.place_food();
        u
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

}

