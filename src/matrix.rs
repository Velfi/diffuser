pub struct Matrix2D {
    cells: Vec<f32>,
    width: usize,
    height: usize,
}

impl Matrix2D {
    pub fn new(height: usize, width: usize) -> Self {
        if height > width {
            println!("Matrix2D height ({}) is greater than Matrix2D width ({}). Are you sure about that?", height, width)
        }

        let length = height * width;
        let cells = (0..length).into_iter().map(|_| 0.0).collect();

        Self {
            cells,
            height,
            width,
        }
    }

    pub fn h(&self) -> usize {
        self.height
    }

    pub fn w(&self) -> usize {
        self.width
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&f32> {
        let index = calculate_index_from_xy(x, y, self.height, self.width);
        self.cells.get(index)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut f32> {
        let index = calculate_index_from_xy(x, y, self.height, self.width);
        self.cells.get_mut(index)
    }

    pub fn get_by_index(&self, index: usize) -> Option<&f32> {
        self.cells.get(index)
    }

    pub fn get_mut_by_index(&mut self, index: usize) -> Option<&mut f32> {
        self.cells.get_mut(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.cells.iter_mut()
    }

    pub fn get_neighbouring_cell(&self, x: usize, y: usize, direction: Direction) -> Option<&f32> {
        if (x > self.width) || (y > self.height) {
            return None;
        }

        let index = calculate_index_from_xy(x, y, self.height, self.width);
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
        let index = calculate_index_from_xy(x, y, self.height, self.width);
        match self.get_neighbour_index(index, direction) {
            Some(neighbour_index) => self.cells.get_mut(neighbour_index),
            _ => None,
        }
    }

    pub fn get_neighbouring_cell_by_index(
        &self,
        index: usize,
        direction: Direction,
    ) -> Option<&f32> {
        match self.get_neighbour_index(index, direction) {
            Some(neighbour_index) => self.cells.get(neighbour_index),
            _ => None,
        }
    }

    pub fn get_neighbouring_cell_mut_by_index(
        &mut self,
        index: usize,
        direction: Direction,
    ) -> Option<&mut f32> {
        match self.get_neighbour_index(index, direction) {
            Some(neighbour_index) => self.cells.get_mut(neighbour_index),
            _ => None,
        }
    }

    fn get_neighbour_index(&self, index: usize, direction: Direction) -> Option<usize> {
        let index = index as isize;
        let width = self.width as isize;
        let height = self.height as isize;

        use Direction::*;
        let neighbour_index = match direction {
            NorthWest => index_to_the_northwest(index, height, width),
            North => index_to_the_north(index, height, width),
            NorthEast => index_to_the_northeast(index, height, width),
            West => index_to_the_west(index, height, width),
            East => index_to_the_east(index, height, width),
            SouthEast => index_to_the_southeast(index, height, width),
            South => index_to_the_south(index, height, width),
            SouthWest => index_to_the_southwest(index, height, width),
        };

        if neighbour_index < 0 || neighbour_index > self.cells.len() as isize {
            None
        } else {
            Some(neighbour_index as usize)
        }
    }
}

pub fn calculate_index_from_xy(x: usize, y: usize, _height: usize, width: usize) -> usize {
    // assert!((0..=width).contains(&x), "calculate_index_from_xy() was passed an x value that was out of range (was {}, should have been in range 0..{})", x, width);
    // assert!((0..=height).contains(&y), "calculate_index_from_xy() was passed a y value that was out of range (was {}, should have been in range 0..{})", y, height);

    x + width * y
}

#[derive(Clone, Copy)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthEast,
    South,
    SouthWest,
}

fn index_is_in_range(index: isize, height: isize, width: isize) -> bool {
    index >= 0 && index < (height * width)
}

fn index_is_in_first_row(index: isize, _height: isize, width: isize) -> bool {
    index >= 0 && index < width
}

fn index_is_in_last_row(index: isize, height: isize, width: isize) -> bool {
    index >= (width * (height - 1)) && index < (width * height)
}

fn index_is_in_first_column(index: isize, height: isize, width: isize) -> bool {
    index_is_in_range(index, height, width) && index % width == 0
}

fn index_is_in_last_column(index: isize, height: isize, width: isize) -> bool {
    index_is_in_range(index, height, width) && (index + 1) % width == 0
}

fn index_to_the_northwest(index: isize, height: isize, width: isize) -> isize {
    match (
        index_is_in_first_column(index, height, width),
        index_to_the_north(index, height, width),
    ) {
        (false, north_index) if north_index != -1 => north_index - 1,
        _ => -1,
    }
}

fn index_to_the_north(index: isize, height: isize, width: isize) -> isize {
    if index_is_in_range(index, height, width) && !index_is_in_first_row(index, height, width) {
        index - width
    } else {
        -1
    }
}

fn index_to_the_northeast(index: isize, height: isize, width: isize) -> isize {
    match (
        index_is_in_last_column(index, height, width),
        index_to_the_north(index, height, width),
    ) {
        (false, north_index) if north_index != -1 => north_index + 1,
        _ => -1,
    }
}

fn index_to_the_west(index: isize, height: isize, width: isize) -> isize {
    if !index_is_in_first_column(index, height, width) {
        index - 1
    } else {
        -1
    }
}

fn index_to_the_east(index: isize, height: isize, width: isize) -> isize {
    if !index_is_in_last_column(index, height, width) {
        index + 1
    } else {
        -1
    }
}

fn index_to_the_southeast(index: isize, height: isize, width: isize) -> isize {
    match (
        index_is_in_last_column(index, height, width),
        index_to_the_south(index, height, width),
    ) {
        (false, north_index) if north_index != -1 => north_index + 1,
        _ => -1,
    }
}

fn index_to_the_south(index: isize, height: isize, width: isize) -> isize {
    if index_is_in_range(index, height, width) && !index_is_in_last_row(index, height, width) {
        index + width
    } else {
        -1
    }
}

fn index_to_the_southwest(index: isize, height: isize, width: isize) -> isize {
    match (
        index_is_in_first_column(index, height, width),
        index_to_the_south(index, height, width),
    ) {
        (false, north_index) if north_index != -1 => north_index - 1,
        _ => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
      | 0  1  2  3  4  5
    __|_________________
    0 | 0  1  2  3  4  5
    1 | 6  7  8  9 10 11
    2 |12 13 14 15 16 17
    */

    #[test]
    fn test_calculate_index_from_xy() {
        let (width, height) = (6, 3);
        let mut expected = 7;
        let mut actual = calculate_index_from_xy(1, 1, width, height);
        assert_eq!(expected, actual);

        expected = 17;
        actual = calculate_index_from_xy(5, 2, width, height);
        assert_eq!(expected, actual);

        expected = 12;
        actual = calculate_index_from_xy(0, 2, width, height);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index_to_the_northwest() {
        let (height, width) = (3, 6);
        let expected = 7;
        let actual = index_to_the_northwest(14, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index_to_the_north() {
        let (height, width) = (3, 6);
        let expected = 3;
        let actual = index_to_the_north(9, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index_to_the_northeast() {
        let (height, width) = (3, 6);
        let expected = 3;
        let actual = index_to_the_northeast(8, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index_to_the_west() {
        let (height, width) = (3, 6);
        let expected = 8;
        let actual = index_to_the_west(9, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index_to_the_east() {
        let (height, width) = (3, 6);
        let expected = 10;
        let actual = index_to_the_east(9, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index_to_the_southeast() {
        let (height, width) = (3, 6);
        let expected = 16;
        let actual = index_to_the_southeast(9, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index_to_the_south() {
        let (height, width) = (3, 6);
        let expected = 15;
        let actual = index_to_the_south(9, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_index_to_the_southwest() {
        let (height, width) = (3, 6);
        let expected = 14;
        let actual = index_to_the_southwest(9, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_index_to_the_northwest() {
        let (height, width) = (3, 6);
        let expected = -1;
        let actual = index_to_the_northwest(3, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_index_to_the_north() {
        let (height, width) = (3, 6);
        let expected = -1;
        let actual = index_to_the_north(3, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_index_to_the_northeast() {
        let (height, width) = (3, 6);
        let expected = -1;
        let actual = index_to_the_northeast(3, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_index_to_the_west() {
        let (height, width) = (3, 6);
        let expected = -1;
        let actual = index_to_the_west(6, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_index_to_the_east() {
        let (height, width) = (3, 6);
        let expected = -1;
        let actual = index_to_the_east(17, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_index_to_the_southeast() {
        let (height, width) = (3, 6);
        let expected = -1;
        let actual = index_to_the_southeast(11, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_index_to_the_south() {
        let (height, width) = (3, 6);
        let expected = -1;
        let actual = index_to_the_south(12, height, width);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_index_to_the_southwest() {
        let (height, width) = (3, 6);
        let expected = -1;
        let actual = index_to_the_southwest(6, height, width);
        assert_eq!(expected, actual);
    }
}
