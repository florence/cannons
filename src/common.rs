use std::cell::Cell;
use std::rc::Rc;

pub type Shared<T> = Rc<Cell<T>>;
#[inline]
pub fn shared<T>(t: T) -> Shared<T> {
    Rc::new(Cell::new(t))
}