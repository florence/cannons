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
    orient: Shared<f64>,
    dir: Vector2<f64>,
    speed: Shared<f64>,
    w: f64,
    h: f64,
    orientation_front_gun: Shared<f64>,
    front_gun: f64,
    orientation_back_gun: Shared<f64>,
    back_gun: f64,
    sw: f64,
    sh: f64,
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

        Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw(
            [0.0, 0.0, self.size * 0.8 * 0.5, self.size * 0.05],
            &ctx.draw_state,
            ctx.transform
                .trans(self.size / 2.0, self.size / 2.0)
                .rot_rad(self.dir.get()),
            g,
        );
    }
}

fn make_gun(x: f64, y: f64) -> (Shared<f64>, Box<UI<Gun>>) {
    let orient = shared(0.0);
    let uigun = Box::new(UI::new(
        Gun {
            dir: orient.clone(),
            size: 100.0,
        },
        x,
        y,
    ));
    (orient, uigun)
}

struct ShipControl {
    dir: Shared<f64>, // radians
    size: f64,
}
impl Component for ShipControl {
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

        Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw(
            [
                self.size * 0.8 * 0.5 * 0.5,
                self.size * 0.05 * 0.05,
                self.size * 0.8 * 0.5 * 0.5,
                self.size * 0.05 * 0.05,
            ],
            &ctx.draw_state,
            ctx.transform
                .trans(self.size / 2.0, self.size / 2.0)
                .rot_rad(self.dir.get()),
            g,
        );
    }
}

fn make_ship_control(x: f64, y: f64) -> (Shared<f64>, Box<UI<ShipControl>>) {
    let orient = shared(0.0);
    let uiship = Box::new(UI::new(
        ShipControl {
            dir: orient.clone(),
            size: 500.0,
        },
        x,
        y,
    ));
    (orient, uiship)
}



const SPEED: f64 = 0.1;

impl Ship {
    pub fn new<T: GameObjectFactory>(fact: &mut T, sw: f64, sh: f64) -> GameObject {
        let x_min = 500.0;
        let (orient_front, uigun_front) = make_gun(500.0, 0.0);
        let (orient_back, uigun_back) = make_gun(500.0, 100.0);
        let (orient_ship, uiship) = make_ship_control(0.0, 0.0);
        let dir = [0.0, 1.0];
        orient_ship.set(vec2_angle(dir));
        let ship = Ship {
            color: [1.0, 0.0, 0.0, 1.0],
            pos: [(sw - x_min) / 2.0, sh / 2.0],
            orient: orient_ship,
            dir: dir,
            speed: shared(SPEED),
            w: 10.0,
            h: 10.0,
            orientation_front_gun: orient_front.clone(),
            front_gun: orient_front.get(),
            orientation_back_gun: orient_back.clone(),
            back_gun: orient_back.get(),
            sw: sw,
            sh: sh,
        };

        let boundship = Box::new(UI::new_bounds(ship, x_min, 0.0, sw - x_min, sh));
        fact.new_gameobject()
            .add(uigun_front)
            .add(uigun_back)
            .add(uiship)
            .add(boundship)
    }
    fn bound<T: Component>(&self, o: T) -> UI<T> {
        let x_min = 500.0;
        UI::new_bounds(o, x_min, 0.0, self.sw - x_min, self.sh)
    }
}
impl Component for Ship {
    fn tick(&mut self, a: &UpdateArgs, w: &mut World) {
        self.dir[0] = self.orient.get().cos();
        self.dir[1] = self.orient.get().sin();
        self.pos = vec2_add(self.pos, vec2_scale(self.dir, self.speed.get()));
        self.pos[0] = f64::max(f64::min(self.pos[0], 640.0 - self.w), 0.0);
        self.pos[1] = f64::max(f64::min(self.pos[1], 480.0 - self.h), 0.0);

        self.front_gun = self.orientation_front_gun.get();
        self.back_gun = self.orientation_back_gun.get();
    }
    fn press(&mut self, b: &Button, w: &mut World) {
        match b {
            &Keyboard(k) => {
                match k {
                    Key::W => self.dir = vec2_max_add(self.dir, [0.0, -SPEED], 1.0),
                    Key::S => self.dir = vec2_max_add(self.dir, [0.0, SPEED], 1.0),
                    Key::A => self.dir = vec2_max_add(self.dir, [-SPEED, 0.0], 1.0),
                    Key::D => self.dir = vec2_max_add(self.dir, [SPEED, 0.0], 1.0),
                    _ => (),
                }
            }
            _ => (),
        }
    }
    fn release(&mut self, b: &Button, w: &mut World) {
        match b {
            &Keyboard(k) => {
                match k {
                    Key::Space => {
                        let r = vec2_angle(self.dir);
                        let front_r = self.front_gun + r;
                        let back_r = self.back_gun + r;
                        let b1 = self.bound(Bullet::new(
                            self.color,
                            vec2_add(self.pos,vec2_scale(self.dir,self.w)),
                            vec2_scale(
                                [front_r.sin(), -front_r.cos()],
                                2.0,
                            ),
                        ));
                        let b2 = self.bound(Bullet::new(
                            self.color,
                            vec2_add(self.pos,vec2_scale(self.dir,-self.w)),
                            vec2_scale([back_r.sin(), -back_r.cos()], 2.0),
                        ));
                        w.spawn_comp(b1);
                        w.spawn_comp(b2);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    fn draw(&mut self, ctx: Context, g: &mut G2d) {
        let transform = ctx.transform.trans(self.pos[0], self.pos[1]).rot_rad(
            vec2_angle(
                self.dir,
            ),
        );
        rectangle(self.color, [0.0, 0.0, self.w, self.h], transform, g);
        rectangle(self.color, [0.0, self.h, self.w, self.h], transform, g);
        rectangle(self.color, [0.0, -self.h, self.w, self.h], transform, g);
        rectangle(
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, (self.w / 4.0), -(self.h)],
            transform
                .append_transform(translate([(self.w / 2.0), (self.h / 2.0) + self.h]))
                .append_transform(rotate_radians(self.front_gun)),
            g,
        );
        rectangle(
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, (self.w / 4.0), -(self.h)],
            transform
                .append_transform(translate([(self.w / 2.0), (self.h / 2.0) - self.h]))
                .append_transform(rotate_radians(self.back_gun)),
            g,
        );
    }
}