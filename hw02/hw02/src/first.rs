use std::mem;
// TODO: everything
#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl BST {
    pub fn new() -> BST {
        BST { root: Link::Empty }
    }

    pub fn insert(&mut self, v: i32) -> bool {
        self.root.insert(v)
    }

    pub fn search(&mut self, v: i32) -> bool {
        self.root.search(v)
    }
}

impl Link {
    fn insert(&mut self, v: i32) -> bool {
        match self {
            Link::Empty => {
                mem::replace(
                    self,
                    Link::More(Box::new(Node {
                        val: v,
                        left: Link::Empty,
                        right: Link::Empty,
                    })),
                );
                return true;
            }
            Link::More(r) => {
                let n = r.as_mut();
                if v == n.val {
                    return false;
                } else if v > n.val {
                    return n.right.insert(v);
                } else {
                    return n.left.insert(v);
                }
            }
        }
    }

    fn search(&mut self, v: i32) -> bool {
        match self {
            Link::Empty => false,
            Link::More(r) => {
                let n = r.as_mut();
                if n.val == v {
                    true
                } else if v > n.val {
                    n.right.search(v)
                } else {
                    n.left.search(v)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_push_pop() {
        let mut m = BST::new();
        assert_eq!(m.insert(1), true);
        assert_eq!(m.search(1), true);

        assert_eq!(m.insert(2), true);
        assert_eq!(m.search(2), true);

        assert_eq!(m.insert(3), true);
        assert_eq!(m.search(3), true);

        assert_eq!(m.search(4), false);
    }
}
