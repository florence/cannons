use vecmath::*;

pub fn vec2_angle(v: Vector2<f64>) -> f64 {
    vec2_angle_between(v,[0.0, 1.0])
}
pub fn vec2_angle_between(a: Vector2<f64>, b: Vector2<f64>) -> f64 {
    -(b[1].atan2(b[0]) - a[1].atan2(a[0]))
    //(vec2_dot(a, b) / (vec2_len(a)*vec2_len(b))).acos()
}

pub fn vec2_max_add(a: Vector2<f64>, b: Vector2<f64>, max: f64) -> Vector2<f64> {
    let v = vec2_add(a,b);
    if vec2_len(v) > max {
        vec2_scale(vec2_normalized(v), max)
    } else {
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float::*;
    #[test]
    fn angle() {
        assert_eq!(vec2_angle([0.0, 1.0]).rad_to_deg(), 0.0);
        assert_eq!(vec2_angle([1.0, 0.0]).rad_to_deg(), -90.0);
        assert_eq!(vec2_angle([0.0, -1.0]).rad_to_deg(), -180.0);
        assert_eq!(vec2_angle([-1.0, 0.0]).rad_to_deg(), 90.0);
        //ug floats
        //assert_eq!(vec2_angle([1.0, 1.0]),f64::_90() / 2.0);
    }
}
