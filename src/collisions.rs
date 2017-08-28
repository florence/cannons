
use vecmath::*;

pub type BoundingBox = Vector4<f64>;
pub fn collides_box(a: BoundingBox, b: BoundingBox) -> bool {
    collides_point(a, [b[0], b[1]]) || collides_point(a, [b[0], b[1] + b[3]]) ||
        collides_point(a, [b[0] + b[2], b[1]]) ||
        collides_point(a, [b[0] + b[2], b[1] + b[3]])
}
pub fn collides_point(b: BoundingBox, p: Vector2<f64>) -> bool {
    p[0] >= b[0] && p[0] <= (b[0] + b[2]) && p[1] >= b[1] && p[1] >= (b[1] + b[3])
}
