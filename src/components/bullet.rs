use piston_window::*;
use vecmath::*;
use components::*;

pub struct Bullet {
    color: [f32; 4],
    pos: Vector2<f64>,
    dir: Vector2<f64>,
}

impl Bullet {
    pub fn new<T: GameObjectFactory>(fact: &mut T,color: [f32; 4], pos: Vector2<f64>, dir: Vector2<f64>) -> GameObject {
        fact.new_gameobject().add(Box::new(Bullet {color,pos,dir}))
    }
}

impl Component for Bullet {
    fn tick(&mut self, args: &UpdateArgs, _: &mut World) {
        self.pos = vec2_add(self.pos, self.dir);
    }
    fn draw(&mut self, c: Context, g: &mut G2d) {
        rectangle(
            self.color, // red
            [self.pos[0], self.pos[1], 2.0, 2.0],
            c.transform,
            g,
        );
    }
}

