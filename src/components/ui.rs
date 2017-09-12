
use components::*;
use piston_window::*;
use vecmath::*;
use collisions::*;
use graphics::math::*;

pub struct UI<T: Component> {
    obj: T,
    bounds: BoundingBox,
}

impl<T: Component> UI<T> {
    pub fn new(obj: T, x: f64, y: f64) -> UI<T> {
        let bb = obj.bounding_box();
        UI {
            obj: obj,
            bounds: [x, y, bb[2], bb[3]],
        }
    }
}

impl<T: Component> Component for UI<T> {
    fn bounding_box(&self) -> BoundingBox {
        self.bounds
    }
    fn draw(&mut self, c: Context, g: &mut G2d) {
        self.obj.draw(
            c.append_transform(translate([self.bounds[0],self.bounds[1]])),
            g,
        )
    }
    fn drag(&mut self, x: f64, y: f64, w: &mut World) {
        if collides_point(self.bounds, [x, y]) {
            self.obj.drag(x - self.bounds[0], y - self.bounds[0], w)
        }
    }
    fn click(&mut self, x: f64, y: f64, w: &mut World) {
        if collides_point(self.bounds, [x, y]) {
            self.obj.click(x - self.bounds[0], y - self.bounds[0], w)
        }
    }
}