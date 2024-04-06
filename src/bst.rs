// 5. Бінарне дерево пошуку. Для заданого масиву ключів (більше 10
// значень, задати випадково – цілі числа з множини [0, 100]) побудувати
// бінарне дерево пошуку, реалізувати всі варіанти обходів (прямий, обернений,
// симетричний). Вивести побудоване дерево і результати обходів.

use std::fmt::Display;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Display + PartialOrd,
{
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut node) => node.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    pub fn preorder(&self) {
        print!("{} ", self.value);
        if let Some(ref node) = self.left {
            node.preorder();
        }
        if let Some(ref node) = self.right {
            node.preorder();
        }
    }

    pub fn inorder(&self) {
        if let Some(ref node) = self.left {
            node.inorder();
        }
        print!("{} ", self.value);
        if let Some(ref node) = self.right {
            node.inorder();
        }
    }

    pub fn postorder(&self) {
        if let Some(ref node) = self.left {
            node.postorder();
        }
        if let Some(ref node) = self.right {
            node.postorder();
        }
        print!("{} ", self.value);
    }
}
