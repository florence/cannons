use vecmath::*;

pub fn vec2_angle(v: Vector2<f64>) -> f64 {
    vec2_angle_between([0.0, 1.0], v)
}
pub fn vec2_angle_between(a: Vector2<f64>, b: Vector2<f64>) -> f64 {
    vec2_dot(a, b).acos()
}

#[cfg(test)]
mod test {
    use super::*;
    use float::*;
    #[test]
    fn angle() {
        assert_eq!(vec2_angle([0.0, 1.0]), 0.0);
        assert_eq!(vec2_angle([0.0, -1.0]), f64::_180());
    }
}