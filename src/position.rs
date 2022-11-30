#[derive(Copy, Clone)]
pub struct Position2(pub f64, pub f64);
impl From<(usize, usize)> for Position2 {
    fn from(p: (usize, usize)) -> Self {
        Position2(p.0 as f64, p.1 as f64)
    }
}
impl From<(f64, f64)> for Position2 {
    fn from(p: (f64, f64)) -> Self {
        Position2(p.0, p.1)
    }
}
impl From<Position3> for Position2 {
    fn from(p: Position3) -> Self {
        Position2(p.0, p.1)
    }
}
impl From<(usize, usize)> for Position3 {
    fn from(p: (usize, usize)) -> Self {
        Position3(p.0 as f64, p.1 as f64, 0.0)
    }
}
#[derive(Copy, Clone)]
pub struct Position3(pub f64, pub f64, pub f64);
impl From<Position2> for Position3 {
    fn from(p: Position2) -> Self {
        Position3(p.0, p.1, 0.0)
    }
}
pub(crate) fn position_to_index(position: Position3, width: usize, height: usize) -> usize {
    let pixel_coords = normalized_to_real(position, width, height);
    to_1d_index(pixel_coords.0, pixel_coords.1, width)
}
pub(crate) fn to_1d_index(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}
pub(crate) fn real_to_normalized<P: Into<Position2>>(
    real_position: P,
    width: usize,
    height: usize,
) -> Position2 {
    let Position2(mut x, mut y) = real_position.into();
    x -= (width as f64)/2.0;
    y -= (height as f64)/2.0;
    x /= (width as f64)/2.0;
    y /= (height as f64)/2.0;
    y *= -1.0;
    Position2(x, y)
}
pub(crate) fn normalized_to_real<P: Into<Position2>>(
    normalized_position: P,
    width: usize,
    height: usize,
) -> (usize, usize) {
    let mut new_position = normalized_position.into();
    new_position.0 *= (width as f64)/2.0;
    new_position.0 += (width as f64)/2.0;
    new_position.1 *= -1.0;
    new_position.1 *= (height as f64)/2.0;
    new_position.1 += (height as f64)/2.0;
    (new_position.0 as usize, new_position.1 as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalized_to_real_test(){
        let result = normalized_to_real((0.5, 0.5), 100, 100);
        assert_eq!(result.0 as u64, 75);
        assert_eq!(result.1 as u64, 25);
    }
    #[test]
    fn real_to_normalized_test(){

        let result = real_to_normalized((75, 25), 100, 100);
        assert_eq!(result.0, 0.5);
        assert_eq!(result.1, 0.5);
    }
}