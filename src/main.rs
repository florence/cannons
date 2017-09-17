extern crate piston_window;
extern crate graphics;
extern crate float;
extern crate piston;
extern crate vecmath;
#[allow(unused_variables)]

mod components;
mod universe;
mod collisions;
mod math;
mod common;

use universe::*;
use piston_window::*;

use components::ship::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [1200, 500])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut u = Universe::new();
    let ship = Ship::new(&mut u, 1200.0, 500.0);
    u.add(ship);
    while let Some(event) = window.next() {
        u = u.handle_event(event, &mut window);
    }
}