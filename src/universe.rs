
use components::*;
use piston_window::*;
use std::collections::linked_list::*;
use piston_window::Input::*;
use piston_window::Button::*;

pub struct Universe {
    components: LinkedList<Box<Component>>,
    mouse_down: bool,
    mouse_x: f64,
    mouse_y: f64,
}
impl Universe {
    pub fn new() -> Universe {
        Universe {
            components: LinkedList::new(),
            mouse_down: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
    pub fn add(&mut self, b: Box<Component>) {
        self.components.push_front(b);
    }
    pub fn handle_event(self, i: Input, window: &mut PistonWindow) -> Self {
        match i {
            Press(button) => self.handle_press(button),
            Release(button) => self.handle_release(button),
            Move(m) => self.handle_move(m),
            Render(_) => self.handle_draw(i, window),
            Update(u) => self.each(|c, w| c.tick(&u, w)),
            _ => self,
        }
    }
    fn handle_draw(self, i: Input, window: &mut PistonWindow) -> Self {
        let mut y = None;
        window.draw_2d(&i, |ctx, g| {
            clear([1.0; 4], g);
            y = Some(self.each(|c, _| c.draw(ctx, g)));
        });
        y.unwrap()
    }
    fn handle_press(mut self, b: Button) -> Self {
        match b {
            Keyboard(_) => self.each(|c, w| c.press(&b, w)),
            Mouse(m) => {
                match m {
                    MouseButton::Left => {
                        self.mouse_down = true;
                        self
                    }
                    _ => self.each(|c, w| c.press(&b, w)),
                }
            }
            Controller(_) => self,
        }
    }
    fn handle_release(self, b: Button) -> Self {
        match b {
            Keyboard(_) => self.each(|c, w| c.release(&b, w)),
            Mouse(m) => self.handle_mouse_release(m),
            Controller(_) => self,
        }
    }
    fn handle_mouse_release(mut self, m: MouseButton) -> Self {
        match m {
            MouseButton::Left => {
                self.mouse_down = false;
                let x = self.mouse_x;
                let y = self.mouse_y;
                self.each(|c, w| c.click(x, y, w))
            }
            _ => self.each(|c, w| c.press(&Mouse(m), w)),
        }
    }
    fn handle_move(mut self, m: Motion) -> Self {
        match m {
            Motion::MouseCursor(x, y) => {
                self.mouse_x = x;
                self.mouse_y = y;
                if self.mouse_down {
                    self.each(|c, w| c.drag(x, y, w))
                } else {
                    self
                }
            }
            _ => self,
        }
    }
    fn each<F>(self, mut f: F) -> Self
    where
        F: FnMut(&mut Component, &mut World) -> (),
    {
        let Universe {
            components,
            mouse_down,
            mouse_x,
            mouse_y,
        } = self;

        let (mut w, mut oc) = World::new(components);
        loop {
            match oc {
                Some(mut c) => {
                    f(&mut *c, &mut w);
                    oc = w.rotate(c);
                }
                None => break,
            }
        }
        let c = w.complete();
        Universe {
            components: c,
            mouse_down: mouse_down,
            mouse_x: mouse_x,
            mouse_y: mouse_y,
        }
    }
}