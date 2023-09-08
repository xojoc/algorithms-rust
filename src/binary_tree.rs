use bumpalo::Bump;

#[derive(Debug)]
struct Node<'a, T: PartialOrd> {
    left: Option<&'a mut Node<'a, T>>,
    right: Option<&'a mut Node<'a, T>>,
    value: T,
}
impl<'a, T: PartialOrd> Node<'a, T> {
    fn new(value: T, arena: &'a Bump) -> &mut Self {
        arena.alloc(Node {
            left: None,
            right: None,
            value,
        })
    }

    fn insert(self: &mut Self, value: T, arena: &'a Bump) {
        if value <= self.value {
            match self.left {
                None => self.left = Some(Node::new(value, arena)),
                Some(ref mut l) => l.insert(value, arena),
            }
        } else {
            match self.right {
                None => self.right = Some(Node::new(value, arena)),
                Some(ref mut r) => r.insert(value, arena),
            }
        }
    }
}
#[derive(Debug)]
pub struct BinaryTree<'a, T: PartialOrd> {
    root: Option<&'a mut Node<'a, T>>,
}

impl<'a, T: PartialOrd> BinaryTree<'a, T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, value: T, arena: &'a Bump) {
        match self.root {
            None => self.root = Some(Node::new(value, arena)),
            Some(ref mut r) => r.insert(value, arena),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_insert() {
        let arena = Bump::new();
        let mut bt: BinaryTree<usize> = BinaryTree::new();
        bt.insert(3, &arena);
        bt.insert(4, &arena);

        println!("Binary tree: {:?}", bt);
    }
}
