use std::cell::RefCell;
use std::rc::Rc;

use super::room::Room;

pub struct Hall {
    pub left: Rc<RefCell<Room>>,
    pub right: Rc<RefCell<Room>>,
}

impl Hall {
    pub fn new() -> Hall {
        // TODO: Implement
        // use dummy value for the initial value
        Hall {
            left: Rc::new(RefCell::new(Room {
                name: "".to_string(),
                contents: vec![],
                halls: vec![],
                wumpus: false,
            })),
            right: Rc::new(RefCell::new(Room {
                name: "".to_string(),
                contents: vec![],
                halls: vec![],
                wumpus: false,
            })),
        }
    }

    /// Given a Room `room`, find the room at the other end of Hall `self`.
    pub fn other(&self, room: &Room) -> Rc<RefCell<Room>> {
        // TODO: Implement
        let lr = &self.left.borrow().name;

        if room.name == *lr {
            return self.right.clone();
        }
        self.left.clone()
    }
}
