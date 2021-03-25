pub struct Vector2<NumericType> {
    pub x: NumericType,
    pub y: NumericType,
}

impl<NumericType> Vector2<NumericType> {
    pub fn new(x: NumericType, y: NumericType) -> Self {
        Vector2 { x, y }
    }
}
