use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width: width,
            height: height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let index = self.get_index(row, col);
            self.cells[index] = Cell::Alive;
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);
                next[index] = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }
        self.cells = next;
    }

    pub fn row_as_string(&self, row: u32) -> Option<String> {
        if row < self.height {
            let mut row_string = String::new();
            let start = self.get_index(row, 0);
            let end = self.get_index(row, self.width);
            let line = &self.cells[start..end];
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                row_string.push(symbol);
            }
            Some(row_string)
        } else {
            None
        }
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_index() {
        let sut = Universe::new(4, 4);

        assert_eq!(sut.get_index(1, 2), 6);
    }

    #[test]
    fn set_cells() {
        let mut sut = Universe::new(1, 2);
        sut.set_cells(&[(0, 1)]);
        assert_eq!(sut.cells, vec![Cell::Dead, Cell::Alive]);
    }

    #[test]
    fn tcik() {
        let mut sut = Universe::new(6, 6);
        sut.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
        let mut expected_universe = Universe::new(6, 6);
        expected_universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
        sut.tick();
        assert_eq!(&sut.cells, &expected_universe.cells);
    }
}
