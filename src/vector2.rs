pub struct Vector2<NumericType> {
    pub x: NumericType,
    pub y: NumericType,
}

impl<NumericType> Vector2<NumericType> {
    pub fn new(x: NumericType, y: NumericType) -> Self {
        Vector2 { x, y }
    }
}

impl<NumericType> Clone for Vector2<NumericType>
where
    NumericType: Clone + Copy,
{
    fn clone(&self) -> Self {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<NumericType> Copy for Vector2<NumericType>
where
    NumericType: Copy {}
