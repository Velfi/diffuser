pub struct Rect<NumericType> {
    width: NumericType,
    height: NumericType,
}

impl<NumericType> Rect<NumericType>
where
    NumericType: Copy,
{
    pub fn new(height: NumericType, width: NumericType) -> Self {
        Rect { width, height }
    }

    pub fn w(&self) -> NumericType {
        self.width
    }

    pub fn h(&self) -> NumericType {
        self.height
    }
}

impl Rect<usize> {
    pub fn contains(&self, x: usize, y: usize) -> bool {
        (0..self.height).contains(&(y)) && (0..self.width).contains(&(x))
    }
}
