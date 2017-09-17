pub mod bullet;
pub mod ship;
pub mod ui;

use piston_window::G2d;
use piston::input::*;
use piston_window::Context;
use std::collections::linked_list::*;
use std::collections::hash_set::*;
use std::iter::*;
use std::any::Any;

use collisions::*;

pub type UUID = u32;

pub struct World {
    prev: LinkedList<GameObject>,
    rest: LinkedList<GameObject>,
    spawned: LinkedList<GameObject>,
    to_destroy: HashSet<UUID>,
    id_counter: UUID,
}

pub trait GameObjectFactory {
    fn new_gameobject(&mut self) -> GameObject;
}
impl GameObjectFactory for World {
    fn new_gameobject(&mut self) -> GameObject {
        let id = self.id_counter;
        self.id_counter += 1;
        GameObject {
            components: LinkedList::new(),
            id,
        }
    }
}

impl World {
    pub fn new(
        mut components: LinkedList<GameObject>,
        id_counter: UUID,
    ) -> (World, Option<GameObject>) {
        let r = components.pop_back();
        let w = World {
            prev: components,
            rest: LinkedList::new(),
            spawned: LinkedList::new(),
            to_destroy: HashSet::new(),
            id_counter: id_counter,
        };
        (w, r)
    }

    pub fn rotate(&mut self, c: GameObject) -> Option<GameObject> {
        self.rest.push_front(c);
        self.prev.pop_back()
    }
    pub fn complete(self) -> (LinkedList<GameObject>, UUID) {
        match self {
            World {
                prev: _,
                rest,
                spawned,
                to_destroy: destroy,
                id_counter,
            } => {
                let obj = rest.into_iter()
                    .filter(|c| !destroy.contains(&c.id))
                    .chain(spawned.into_iter())
                    .collect::<LinkedList<_>>();
                (obj, id_counter)
            }
        }
    }
}

impl World {
    pub fn spawn(&mut self, comp: GameObject) -> () {
        self.spawned.push_front(comp);
    }
    pub fn spawn_comp<T: Component>(&mut self, comp: T) -> () {
        let go = self.new_gameobject().add(Box::new(comp));
        self.spawn(go);
    }
    pub fn destroy(&mut self, comp: &GameObject) {
        self.to_destroy.insert(comp.id);
    }
    pub fn collisions(&self, bb: BoundingBox) -> LinkedList<&GameObject> {
        self.prev
            .iter()
            .chain(self.rest.iter())
            .filter(|c| collides_box(c.bounding_box(), bb))
            .collect::<LinkedList<_>>()
    }
}

pub trait Component: 'static {
    fn draw(&mut self, c: Context, g: &mut G2d) {}
    fn tick(&mut self, &UpdateArgs, &mut World) {}
    fn press(&mut self, &Button, &mut World) {}
    fn release(&mut self, &Button, &mut World) {}
    fn click(&mut self, x: f64, y: f64, &mut World) {}
    fn drag(&mut self, x: f64, y: f64, &mut World) {}
    fn destroy(&mut self, &mut World) {}

    fn bounding_box(&self) -> BoundingBox {
        [0.0; 4]
    }

    fn collidable(&self) -> bool {
        false
    }
}

// These just don't work unless all components are
// completely independent
pub struct GameObject {
    pub components: LinkedList<Box<Component + 'static>>,
    pub id: UUID,
}

impl GameObject {
    fn add(mut self, comp: Box<Component + 'static>) -> GameObject {
        self.components.push_front(comp);
        self
    }
    fn get<'a, T: Component + 'static>(&'a self) -> Option<&'a T> {
        for x in self.components.iter() {
            if let Some(a) = (x as &Any).downcast_ref::<T>() {
                return Some(a);
            }
        }
        return None;
    }
    fn identify(&self) -> UUID {
        self.id
    }
}

impl Component for GameObject {
    fn draw(&mut self, c: Context, g: &mut G2d) {
        for comp in self.components.iter_mut() {
            comp.draw(c, g);
        }
    }
    fn tick(&mut self, args: &UpdateArgs, w: &mut World) {
        for comp in self.components.iter_mut() {
            comp.tick(args, w);
        }
    }
    fn press(&mut self, b: &Button, w: &mut World) {
        for comp in self.components.iter_mut() {
            comp.press(b, w);
        }
    }
    fn release(&mut self, b: &Button, w: &mut World) {
        for comp in self.components.iter_mut() {
            comp.release(b, w);
        }
    }
    fn drag(&mut self, x: f64, y: f64, w: &mut World) {
        for comp in self.components.iter_mut() {
            comp.drag(x, y, w);
        }
    }
    fn click(&mut self, x: f64, y: f64, w: &mut World) {
        for comp in self.components.iter_mut() {
            comp.click(x, y, w);
        }
    }
    fn destroy(&mut self, w: &mut World) {
        for comp in self.components.iter_mut() {
            comp.destroy(w);
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        let mut bb = [0.0; 4];
        for comp in self.components.iter() {
            let b2 = comp.bounding_box();
            bb[0] = f64::max(bb[0], b2[0]);
            bb[1] = f64::max(bb[1], b2[1]);
            bb[2] = f64::max(bb[2], b2[2]);
            bb[3] = f64::max(bb[3], b2[3]);
        }
        return bb;
    }

    fn collidable(&self) -> bool {
        for comp in self.components.iter() {
            if comp.collidable() {
                return true;
            }
        }
        return false;
    }
}