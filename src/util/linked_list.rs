use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fmt::{format, Debug},
    ptr::{self, null, null_mut},
};

// #[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    pub size: usize,
}

type Link<T> = *mut Node<T>;

#[derive(PartialEq, Eq)]
pub struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                value,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.size += 1;
            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                self.size -= 1;
                Some(head.value)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.value) }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }

    pub unsafe fn set_head(&mut self, new_head: *mut Node<T>) {
        self.head = new_head
    }

    pub unsafe fn get_raw_reference(&self, index: usize) -> *mut Node<T> {
        if index > self.size - 1 {
            return ptr::null_mut();
        }

        if self.head.is_null() {
            return ptr::null_mut();
        }

        let mut self_ptr = self.head;

        for i in 0..index {
            self_ptr = (*self_ptr).next;
        }

        return self_ptr;
    }
}

/* From Vec */
impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut ll = LinkedList::new();

        for val in vec {
            ll.push(val);
        }

        return ll;
    }
}

/* From LinkedList */
impl<T: Copy> From<&LinkedList<T>> for Vec<T> {
    fn from(ll: &LinkedList<T>) -> Self {
        unsafe {
            let mut vec = Vec::new();
            let mut ll_ptr = ll.head;
            for i in 0..ll.size {
                if ll_ptr != null_mut() {
                    let val = ll_ptr.as_ref().ok_or("panic!").unwrap().value;
                    vec.push(val);
                    ll_ptr = (*ll_ptr).next;
                } else {
                    break;
                }
            }

            return vec;
        }
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            if self.size != other.size {
                return false;
            }

            if self.size == 0 {
                return true;
            }

            let mut self_ptr = self.head;
            let mut other_ptr = other.head;

            for i in 0..self.size {
                if self_ptr != null_mut() {
                    let self_val = &(*self_ptr).value;
                    let other_val = &(*other_ptr).value;

                    if self_val != other_val {
                        return false;
                    }

                    self_ptr = (*self_ptr).next;
                    other_ptr = (*other_ptr).next;
                } else {
                    break;
                }
            }

            return true;
        }
    }
}

impl<T: PartialEq> Eq for LinkedList<T> {}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size == 0 {
            write!(f, "{{ }}");
        }

        let mut output: Vec<String> = Vec::with_capacity(self.size + 2);
        output[0] = "{".to_owned();
        unsafe {
            let mut self_ptr = self.head;
            for i in 0..self.size {
                if self_ptr != null_mut() && (*self_ptr).next != null_mut() {
                    let val = self_ptr.as_ref().ok_or("panic!").unwrap();
                    output[i + 1] = format!("{:?}->", val);
                    self_ptr = (*self_ptr).next;
                } else if self_ptr != null_mut() {
                    let val = self_ptr.as_ref().ok_or("panic!").unwrap();
                    output[i + 1] = format!("{:?}", val);
                    self_ptr = (*self_ptr).next;
                } else {
                    break;
                }
            }
        }
        output[self.size + 1] = "}".to_owned();
        write!(f, "{:?}", &output.concat())
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ value: {:?} }}", self.value)
    }
}

/* Drop */
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.value
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ll;

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
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn two_pointers() {
        let actual = ll!(1, 2, 3);

        unsafe {
            let mut ptr_one = actual.get_raw_reference(0);
            let mut ptr_two = actual.get_raw_reference(1);

            let temp = (*ptr_two).next;
            (*ptr_two).next = ptr_one;
            (*ptr_one).next = temp;
        }

        let expected = ll!(1, 3, 2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.peek(), Some(&2));
    }

    #[test]
    fn from_vec() {
        let vec = vec![1, 2, 3];
        let actual = LinkedList::from(vec);
        let mut expected = LinkedList::new();

        expected.push(1);
        expected.push(2);
        expected.push(3);

        assert_eq!(actual.peek(), expected.peek());

        // assert_eq!(actual_linky_poo, expected_linky_poo);
    }

    #[test]
    fn vec_macro() {
        let mut actual = ll![1, 2, 3];
        let mut expected = LinkedList::new();
        expected.push(1);
        expected.push(2);
        expected.push(3);

        assert_eq!(actual, expected);
    }

    // #[test]
    // fn clone() {
    //     let original = ll![1, 2, 3];
    //     let cloned = original.clone();
    //
    //     assert_eq!(original, cloned);
    // }
}
