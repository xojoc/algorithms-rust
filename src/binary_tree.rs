use std::cell::RefCell;

use bumpalo::Bump;
#[derive(Debug)]
struct Node<'a, T: PartialOrd> {
    left: Option<&'a mut Node<'a, T>>,
    right: Option<&'a mut Node<'a, T>>,
    value: T,
}
impl<'a, 'b: 'a, T: PartialOrd> Node<'a, T> {
    fn new(arena: &'a mut Bump, value: T) -> &'b mut Self {
        arena.alloc(Node {
            left: None,
            right: None,
            value,
        })
    }

    fn insert(self: &'b mut Self, arena: &'a mut Bump, value: T) {
        if value <= self.value {
            match self.left {
                None => self.left = Some(Node::new(arena, value)),
                Some(ref mut l) => {
                    if l.value < value {
                        let node = Node::new(arena, value);
                        node.left = Some(l);
                        *l = node;
                    } else {
                        l.insert(arena, value);
                    }
                }
            }
        } else {
            match self.right {
                None => self.right = Some(Node::new(arena, value)),
                Some(ref mut r) => {
                    if r.value > value {
                        let node = Node::new(arena, value);
                        node.right = Some(r);
                        *r = node;
                    } else {
                        r.insert(arena, value);
                    }
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct BinaryTree<'a, T: PartialOrd> {
    arena: Bump,
    root: RefCell<Option<&'a mut Node<'a, T>>>,
}

impl<'a, 'b: 'a, T: PartialOrd> BinaryTree<'a, T> {
    pub fn new() -> Self {
        BinaryTree {
            arena: Bump::new(),
            root: RefCell::new(None),
        }
    }

    pub fn insert(&'b mut self, value: T) {
        let mut root = self.root.borrow_mut();

        if root.is_none() {
            root.replace(Node::new(&mut self.arena, value));
        } else {
            let mut root_root = root;
            // let x: () = root_root;

            // if let Some(root_root_root) = root_root {
            // let x: () = root_root_root;
            // root_root_root.insert(&mut self.arena, value);
            // }
        }
        // match *self.root.borrow_mut() {
        // None => self.root = RefCell::new(Some(Node::new(&mut self.arena, value))),
        // Some(ref mut r) => r.insert(&mut self.arena, value),
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_insert() {
        let mut bt: BinaryTree<usize> = BinaryTree::new();
        {
            bt.insert(3);
        }
        {
            // bt.insert(4);
        }

        // println!("Binary tree: {:?}", bt);
    }
}
