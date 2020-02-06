use std::cmp::Ordering;
use std::mem;
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct BST<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Node<T> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    fn new(e: T) -> Node<T> {
        Node {
            val: e,
            left: None,
            right: None,
        }
    }
}

trait InsertSearch<T> {
    fn insert(&mut self, e: T) -> bool;
    fn search(&mut self, e: T) -> bool;
}

impl<T> BST<T> {
    fn new() -> BST<T> {
        BST { root: None }
    }
}

impl<T: Ord> InsertSearch<T> for BST<T> {
    fn insert(&mut self, e: T) -> bool {
        self.root.insert(e)
    }
    fn search(&mut self, e: T) -> bool {
        self.root.search(e)
    }
}

impl<T: Ord> InsertSearch<T> for Link<T> {
    fn insert(&mut self, e: T) -> bool {
        match self {
            None => {
                mem::replace(self, Some(Box::new(Node::new(e))));
                return true;
            }
            Some(v) => {
                let n = v.as_mut();
                match n.val.cmp(&e) {
                    Ordering::Equal => {
                        return false;
                    }
                    Ordering::Less => {
                        return n.left.insert(e);
                    }
                    Ordering::Greater => {
                        return n.right.insert(e);
                    }
                }
            }
        }
    }
    fn search(&mut self, e: T) -> bool {
        match self {
            None => false,
            Some(v) => {
                let n = v.as_mut();
                match n.val.cmp(&e) {
                    Ordering::Equal => true,
                    Ordering::Less => n.left.search(e),
                    Ordering::Greater => n.right.search(e),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{InsertSearch, BST};

    #[test]
    fn test_push_pop() {
        let mut m = BST::<i32>::new();
        assert_eq!(m.insert(1), true);
        assert_eq!(m.search(1), true);

        assert_eq!(m.insert(2), true);
        assert_eq!(m.search(2), true);

        assert_eq!(m.insert(3), true);
        assert_eq!(m.search(3), true);

        assert_eq!(m.search(4), false);
    }
}
