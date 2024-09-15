use std::rc::Rc;
use std::cell::{Ref, RefCell,RefMut};


type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        let node = Node {
            data,
            next: None,
            prev: None,
        };

        Rc::new(RefCell::new(node))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }

    pub fn push_front(&mut self, data: T){
        let new_node = Node::new(data);
        match self.head.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T>{
        self.head.take().map(|old_head|{
            match old_head.borrow_mut().next.take() {
                None => {
                    self.tail = None;
                },
                Some(new_node) => {
                    new_node.borrow_mut().prev = None;
                    self.head = Some(new_node);
                }
            }
            // I do not understand this
           Rc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    pub fn peek_fron(&mut self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node|{
            Ref::map(node.borrow(), |node| &node.data)
        })
    }

    pub fn push_back(&mut self, data: T) {
        let new_tail = Node::new(data);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.data)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.data)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.data)
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self){
        while self.pop_front().is_some(){}
    }
}
