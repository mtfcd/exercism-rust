use std::iter::FromIterator;

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { next: None, value }
    }
}
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut l = 0;
        let mut next = &self.head;
        while let Some(node) = next {
            l += 1;
            next = &node.next
        }
        l
    }

    pub fn push(&mut self, _element: T) {
        let mut append_point = &mut self.head;

        while let Some(node) = append_point {
            append_point = &mut node.next
        }

        *append_point = Some(Box::new(Node::new(_element)));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None
        }
        let mut next = &mut self.head;
        while next.as_ref().unwrap().next.is_some() {
            next = &mut next
                .as_mut()
                .unwrap()
                .next;
        }
        next.take().map(|n| {
            n.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_none() {
            return None
        }
        let mut next = &self.head;
        while next.as_ref().unwrap().next.is_some() {
            next = &next
                .as_ref()
                .unwrap()
                .next;
        }
        next.as_ref().map(|n| {
            &n.value
        })
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {

        let mut prev = None;
        let mut head = self.head;
        // self.head -> a (value, Some(b)) -> b (value, Some(c)) -> c (value, None)
        // a = self.head; self.head = &a.next; a.next = None;
        // a | head -> b -> c
        // head = &b.next; b.next = &a;
        // a <- b | head -> c
        // c.next = &b;
        // a <- b <- c <- head
        while let Some(a) = head {
            self.head = a.next.take();
            a_next.next = prev;
            prev = a;
            a = &mut self.head;
        }
        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut s = Self::new();
        for i in _iter {
            s.push(i);
        }
        s
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
