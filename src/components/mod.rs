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

type UUID = u32;

pub struct World {
    prev: LinkedList<Box<Component>>,
    rest: LinkedList<Box<Component>>,
    spawned: LinkedList<Box<Component>>,
    to_destroy: HashSet<UUID>,
}

impl World {
    pub fn new(mut components: LinkedList<Box<Component>>) -> (World, Option<Box<Component>>) {
        let r = components.pop_back();
        let w = World {
            prev: components,
            rest: LinkedList::new(),
            spawned: LinkedList::new(),
            to_destroy: HashSet::new(),
        };
        (w, r)
    }
    pub fn rotate(&mut self, c: Box<Component>) -> Option<Box<Component>> {
        self.rest.push_front(c);
        self.prev.pop_back()
    }
    pub fn complete(self) -> LinkedList<Box<Component>> {
        match self {
            World {
                prev: _,
                rest,
                spawned,
                to_destroy: destroy,
            } => {
                rest.into_iter()
                    .filter(|c| match c.identify() {
                        Some(id) => !destroy.contains(&id),
                        None => true,
                    })
                    .chain(spawned.into_iter())
                    .collect::<LinkedList<_>>()
            }
        }
    }
}

impl World {
    pub fn spawn(&mut self, comp: Box<Component>) -> () {
        self.spawned.push_front(comp);
    }
    pub fn destroy(&mut self, comp: Box<&Component>) {
        if let Some(id) = comp.identify() {
            self.to_destroy.insert(id);
        }
    }
    pub fn collisions(&self, bb: BoundingBox) -> LinkedList<&Component> {
        self.prev
            .iter()
            .chain(self.rest.iter())
            .filter(|c| collides_box(c.bounding_box(), bb))
            .map(|c| &**c)
            .collect::<LinkedList<_>>()
    }
}


pub trait Component: Any {
    fn draw(&mut self, Context, &mut G2d) {}
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
    fn identify(&self) -> Option<UUID> {
        None
    }
}

// These just don't work unless all components are
// completely independent
pub struct GameObject {
    components: LinkedList<Box<Component>>,
    id: UUID,
}

impl GameObject {
    fn new(id: UUID) -> GameObject {
        GameObject {
            components: LinkedList::new(),
            id,
        }
    }
    fn add(mut self, comp: Box<Component>) -> GameObject {
        self.components.push_front(comp);
        self
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

    fn identify(&self) -> Option<UUID> {
        Some(self.id)
    }
}