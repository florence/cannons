use components::*;
use vecmath::*;
use piston_window::*;
use super::bullet::*;
use super::ui::*;
use math::*;
use graphics::math::*;
use piston_window::Button::*;
use common::*;

pub struct Ship {
    color: [f32; 4],
    pos: Vector2<f64>,
    dir: Vector2<f64>,
    w: f64,
    h: f64,
    orientation: Shared<f64>,
}

struct Gun {
    dir: Shared<f64>, // radians
    size: f64,
}
impl Component for Gun {
    fn bounding_box(&self) -> BoundingBox {
        [0.0, 0.0, self.size, self.size]
    }
    fn drag(&mut self, x: f64, y: f64, _: &mut World) {
        let v = vec2_sub(
            [self.size, self.size],
            [x + (self.size / 2.0), y + (self.size / 2.0)],
        );
        let a = vec2_angle(v);
        if !a.is_nan() {
            self.dir.set(a);
        }
    }
    fn draw(&mut self, ctx: Context, g: &mut G2d) {
        let bounds = [0.0, 0.0, self.size, self.size];
        Rectangle::new([1.0; 4]).draw(bounds, &ctx.draw_state, ctx.transform, g);
        Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 1.0).draw(
            bounds,
            &ctx.draw_state,
            ctx.transform,
            g,
        );
    }
}

impl Ship {
    pub fn new<T: GameObjectFactory>(fact: &mut T) -> GameObject {
        let orient = shared(0.0);
        let uigun = Box::new(UI::new(
            Gun {
                dir: orient.clone(),
                size: 100.0,
            },
            0.0,
            0.0,
        ));
        let ship = Box::new(Ship {
            color: [1.0, 0.0, 0.0, 1.0],
            pos: [300.0; 2],
            dir: [1.0; 2],
            w: 10.0,
            h: 10.0,
            orientation: orient.clone(),
        });
        // TODO get ID's working
        fact.new_gameobject().add(uigun).add(ship)
    }
}
impl Component for Ship {
    fn drag(&mut self, x: f64, y: f64, w: &mut World) {
        self.dir = vec2_normalized(vec2_sub([x, y], self.pos));
    }
    fn tick(&mut self, a: &UpdateArgs, w: &mut World) {
        self.pos = vec2_add(self.pos, self.dir);
        self.pos[0] = f64::max(f64::min(self.pos[0], 640.0 - self.w), 0.0);
        self.pos[1] = f64::max(f64::min(self.pos[1], 480.0 - self.h), 0.0);
    }
    fn click(&mut self, x: f64, y: f64, w: &mut World) {}
    fn release(&mut self, b: &Button, w: &mut World) {
        match b {
            &Keyboard(k) => {
                match k {
                    Space => {
                        let bullet = Bullet::new(
                            w,
                            self.color,
                            self.pos,
                            vec2_scale(
                                [self.orientation.get().sin(), -self.orientation.get().cos()],
                                2.0,
                            ),
                        );
                        w.spawn(bullet);

                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    fn draw(&mut self, ctx: Context, g: &mut G2d) {
        rectangle(
            self.color,
            [self.pos[0], self.pos[1], self.w, self.h],
            ctx.transform,
            g,
        );
        rectangle(
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, (self.w / 4.0), -(self.h)],
            ctx.append_transform(translate(
                [self.pos[0] + (self.w / 2.0), self.pos[1] + (self.h / 2.0)],
            )).append_transform(rotate_radians(self.orientation.get()))
                .transform,
            g,
        );
    }
}