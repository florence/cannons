use vecmath::*;

pub type BoundingBox = Vector4<f64>;

pub fn bb_x0(b: BoundingBox) -> f64 {
    b[0]
}
pub fn bb_y0(b: BoundingBox) -> f64 {
    b[1]
}
pub fn bb_x1(b: BoundingBox) -> f64 {
    b[0] + b[2]
}
pub fn bb_y1(b: BoundingBox) -> f64 {
    b[1] + b[3]
}

pub fn collides_box(a: BoundingBox, b: BoundingBox) -> bool {
    collides_point(a, [b[0], b[1]]) || collides_point(a, [b[0], b[1] + b[3]]) ||
        collides_point(a, [b[0] + b[2], b[1]]) ||
        collides_point(a, [b[0] + b[2], b[1] + b[3]])
}
pub fn collides_point(b: BoundingBox, p: Vector2<f64>) -> bool {
    p[0] >= bb_x0(b) && p[0] <= bb_x1(b) && p[1] >= bb_y0(b) && p[1] <= bb_y1(b) 
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn coords(){
        let m = [0.0,1.0,2.0,3.0];
        assert_eq!(0.0,bb_x0(m));
        assert_eq!(1.0,bb_y0(m));
        assert_eq!(2.0,bb_x1(m));
        assert_eq!(4.0,bb_y1(m));
    }
    #[test]
    fn test_point(){
        assert!(collides_point([0.0,0.0,10.0,10.0],[1.0,1.0]));
    }
}