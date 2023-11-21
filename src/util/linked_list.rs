use std::{cell::RefCell, borrow::BorrowMut};

#[derive(Debug, PartialEq, Eq)]
pub struct LinkedList<T> {
    pub head: Link<T>,
}

type Link<T> = RefCell<Option<Box<Node<T>>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            next: RefCell::from(None)
        }
    }

    pub fn newLink(value: T) -> Link<T> {
        RefCell::from(Some(Box::new(Node::new(value))))
    }

    pub fn emptyLink() -> Link<T> {
        RefCell::from(None)
    }

    pub fn append(&mut self, value: T) {
        match *self.next.borrow_mut() {
            None => {
                self.next = Node::newLink(value);
            }
            Some(mut next) => next.append(value),
        }
    }
}

#[macro_export]
macro_rules! ll {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_ll= LinkedList::new();
            $(
                temp_ll.append($x);
            )*
            temp_ll
        }
    };
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        return LinkedList { head: Node::emptyLink() };
    }

    pub fn push(&mut self, value: T) {
        let node = Box::new(Node {
            value,
            next: RefCell::from(self.head.take()),
        });

        self.head = RefCell::from(Some(node));
    }

    pub fn append(&mut self, value: T) {
        match *self.head.borrow_mut() {
            None => {
                let new_node = Node::newLink(value);
                self.head = new_node;
            }
            Some(mut node) => node.append(value),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.borrow_mut().map(|n| &n.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.borrow_mut().map(|mut n| &mut n.value)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        let mut node = self.iter_mut();
        // probably need to be smarter about this
        for _ in 0..index {
            node.next();
        }

        return node.next;
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.borrow_mut().map(|n| &*n),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.get_mut().as_deref_mut(),
        }
    }
}

/* Into Iter */

#[derive(Debug)]
pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/* Immutable Iterator*/

#[derive(Debug)]
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // self.next.map(|n| {
        //     self.next = n.next.borrow().as_deref();
        //     &n.value
        // })
        match self.next.borrow_mut() {
            None => None,
            Some(next) => {
                self.next = next.next.borrow_mut().as_deref();
                Some(&next.value)
            }
        }
    }
}

/* Mutable Iterator*/

#[derive(Debug)]
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.borrow_mut().as_deref_mut();
            &mut n.value
        })
    }
}

/* From Vec */
impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut ll = LinkedList::new();

        for val in vec {
            ll.append(val);
        }

        return ll;
    }
}

/* From LinkedList */
impl<T> From<LinkedList<T>> for Vec<T> {
    fn from(ll: LinkedList<T>) -> Self {
        let mut vec = Vec::new();

        for val in ll.into_iter() {
            vec.push(val);
        } 

        return vec;
    }
}

/* Drop */
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

/* Clone */ 
impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        Self { head: self.head.clone() }
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), next: self.next.clone() }
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basics() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn two_pointers() {}

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn get_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let actual = list.get_mut(1).map(|n| n.value);
        assert_eq!(actual, Some(2));
    }

    #[test]
    fn get_iter_big() {
        let mut list = LinkedList::new();
        list.push(10);
        list.push(20);
        list.push(30);
        list.push(100);
        list.push(200);
        list.push(300);
        list.push(1000);
        list.push(2000);
        list.push(3000);

        let actual = list.get_mut(5).map(|n| n.value);
        assert_eq!(actual, Some(100));
    }

    #[test]
    fn from_vec() {
        let vecy_poo = vec![1, 2, 3];
        let actual_linky_poo = LinkedList::from(vecy_poo);
        let mut expected_linky_poo = LinkedList::new();

        expected_linky_poo.push(3);
        expected_linky_poo.push(2);
        expected_linky_poo.push(1);

        assert_eq!(actual_linky_poo, expected_linky_poo);
    }

    #[test]
    fn vec_macro() {
        let mut actual = ll![1, 2, 3];
        let mut expected = LinkedList::new();
        expected.push(3);
        expected.push(2);
        expected.push(1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn clone() {
        let original = ll![1, 2, 3];
        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
