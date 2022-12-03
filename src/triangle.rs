use crate::position::{Position2, Position3};

#[derive(Copy, Clone)]
pub struct Triangle(pub Position3, pub Position3, pub Position3);
impl From<(f64, f64, f64, f64, f64, f64)> for Triangle {
    fn from(triangle: (f64, f64, f64, f64, f64, f64)) -> Self {
        Triangle(
            Position3(triangle.0, triangle.1, 0.0),
            Position3(triangle.2, triangle.3, 0.0),
            Position3(triangle.4, triangle.5, 0.0),
        )
    }
}
impl From<(f64, f64, f64, f64, f64, f64, f64, f64, f64)> for Triangle {
    fn from(triangle: (f64, f64, f64, f64, f64, f64, f64, f64, f64)) -> Self {
        Triangle(
            Position3(triangle.0, triangle.1, triangle.2),
            Position3(triangle.3, triangle.4, triangle.5),
            Position3(triangle.6, triangle.7, triangle.8),
        )
    }
}
impl Triangle {
    pub fn area(self) -> f64 {
        let x1 = self.0 .0;
        let x2 = self.1 .0;
        let x3 = self.2 .0;
        let y1 = self.0 .1;
        let y2 = self.1 .1;
        let y3 = self.2 .1;
        #[cfg(feature = "std")]
        {
            ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) as f64 / 2.0).abs()
        }
        
        #[cfg(not(feature = "std"))]
        {
            let result = (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) as f64 / 2.0;
            if result < 0.0 { -result } else { result }
        }
    }
    pub fn contains_point<P: Into<Position2>>(self, point: P) -> bool {
        let Triangle(Position3(x1, y1, _), Position3(x2, y2, _), Position3(x3, y3, _)) = self;
        let Position2(x, y) = point.into();

        let a = Self::area(Self::from((x1, y1, x2, y2, x3, y3)));
        let a1 = Self::area(Self::from((x, y, x2, y2, x3, y3)));
        let a2 = Self::area(Self::from((x1, y1, x, y, x3, y3)));
        let a3 = Self::area(Self::from((x1, y1, x2, y2, x, y)));
        a == (a1 + a2 + a3)
    }
}
