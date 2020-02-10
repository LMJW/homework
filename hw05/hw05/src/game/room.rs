use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {
    // TODO: Implement the necessary methods for Rooms.

    pub fn neighbors_string(&self) -> String {
        // TODO: Implement
        let mut ret = String::new();

        let cur_room = &self.name;
        for h in &self.halls {
            let n = &h.left.borrow().name;
            let r = &h.right.borrow().name;
            if n == cur_room {
                ret.push_str(r.as_str());
            } else {
                ret.push_str(n.as_str());
            }
            ret.push_str(" ");
        }

        ret
    }
}
