pub struct Matrix2D {
    cells: Vec<f32>,
    width: usize,
    height: usize,
}

impl Matrix2D {
    pub fn new(height: usize, width: usize) -> Self {
        let length = height * width;
        let cells = (0..length).into_iter().map(|_| 0.0).collect();

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn w(&self) -> usize {
        self.width
    }

    pub fn h(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&f32> {
        let index = index_calculator(x, y, self.width);
        self.cells.get(index)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut f32> {
        let index = index_calculator(x, y, self.width);
        self.cells.get_mut(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &f32)> {
        let width = self.width;

        self.cells.iter().enumerate().map(move |(index, value)| {
            let (x, y) = xy_calculator(index, width);
            (x, y, value)
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut f32)> {
        let width = self.width;

        self.cells
            .iter_mut()
            .enumerate()
            .map(move |(index, value)| {
                let (x, y) = xy_calculator(index, width);
                (x, y, value)
            })
    }

    pub fn get_neighbouring_cell(&self, x: usize, y: usize, direction: Direction) -> Option<&f32> {
        let index = index_calculator(x, y, self.width);
        self.get_neighbour_index(index, direction)
            .map(|neighbour_index| self.cells.get(neighbour_index))
            .flatten()
    }

    pub fn get_neighbouring_cell_mut(
        &mut self,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> Option<&mut f32> {
        let index = index_calculator(x, y, self.width);
        match self.get_neighbour_index(index, direction) {
            Some (neighbour_index) => self.cells.get_mut(neighbour_index),
            _ => None
        }
    }

    fn get_neighbour_index(&self, index: usize, direction: Direction) -> Option<usize> {
        let index = index as isize;
        let width = self.width as isize;

        use Direction::*;
        let neighbour_index = match direction {
            NorthWest => index_to_the_northwest(index, width),
            North => index_to_the_north(index, width),
            NorthEast => index_to_the_northeast(index, width),
            West => index_to_the_west(index, width),
            East => index_to_the_east(index, width),
            SouthEast => index_to_the_southeast(index, width),
            South => index_to_the_south(index, width),
            SouthWest => index_to_the_southwest(index, width),
        };

        if neighbour_index < 0 || neighbour_index > self.cells.len() as isize {
            None
        } else {
            Some(neighbour_index as usize)
        }
    }
}

fn index_calculator(x: usize, y: usize, width: usize) -> usize {
    x + width * y
}

fn xy_calculator(index: usize, width: usize) -> (usize, usize) {
    (index / width, index % width)
}

enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthEast,
    South,
    SouthWest,
}

fn index_to_the_northwest(index: isize, width: isize) -> isize {
    if index % width == 0 {
        -1
    } else {
        index - 1 - width
    }
}

fn index_to_the_north(index: isize, width: isize) -> isize {
    index - width
}

fn index_to_the_northeast(index: isize, width: isize) -> isize {
    if (index + 1) % width == 0 {
        -1
    } else {
        index + 1 - width
    }
}

fn index_to_the_west(index: isize, width: isize) -> isize {
    if index % width == 0 {
        -1
    } else {
        index - 1
    }
}

fn index_to_the_east(index: isize, width: isize) -> isize {
    if (index + 1) % width == 0 {
        -1
    } else {
        index + 1
    }
}

fn index_to_the_southeast(index: isize, width: isize) -> isize {
    if (index + 1) % width == 0 {
        -1
    } else {
        index + 1 + width
    }
}

fn index_to_the_south(index: isize, width: isize) -> isize {
    index - width
}

fn index_to_the_southwest(index: isize, width: isize) -> isize {
    if index % width == 0 {
        -1
    } else {
        index - 1 + width
    }
}
