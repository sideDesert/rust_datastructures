use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>
}

pub struct List<T> {
    head: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{ head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T>{
        // let new_next = self.head.as_ref().map(|node| Rc::clone(node));
        let new_next = self.head.as_ref().map(Rc::clone);

        List {
            head: Some(
                Rc::new( Node{ data: elem, next: new_next })
            )
        }
    }

    pub fn tail(&self) -> List<T> {
        let head = self.head.as_ref().and_then(|node| node.next.clone());
        List{ head }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

// For Iterators
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}
