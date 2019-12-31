use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(v: T, n: Option<Box<Node<T>>>) -> Node<T> {
        Node { data: v, next: n }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut num = 0;
        let mut node = &self.head;
        loop {
            match node {
                None => {
                    break;
                }
                Some(v) => {
                    num += 1;
                    node = &v.next;
                }
            }
        }
        num
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node::new(_element, self.head.take()));
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(v) => {
                self.head = v.next;
                Some(v.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(v) => Some(&v.data),
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        while self.peek().is_some() {
            rev_list.push(self.pop().unwrap());
        }
        rev_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        _iter.into_iter().for_each(|x| list.push(x));
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: Copy,
{
    fn into(self) -> Vec<T> {
        let mut vec = vec![];
        let mut head = &self.head;
        loop {
            match head {
                None => {
                    break;
                }
                Some(v) => {
                    head = &v.next;
                    vec.push(v.data);
                }
            }
        }
        vec.reverse();
        vec
    }
}
