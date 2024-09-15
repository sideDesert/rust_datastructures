type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>
}

#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self{
        Stack{
            head: None
        }
    }

    pub fn push(&mut self, data: T){
        let new_node = Node{
            next: self.head.take(),
            data
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }
    
    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<T> {
        Iter{
            next: self.head.as_deref()
        }
    }
    pub fn iter_mut(&mut self) -> MutIter<T> {
        MutIter{
            next: self.head.as_deref_mut()
        }
    }
}

// Iterator Definitions
pub struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>
}
impl<'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

pub struct MutIter<'a,T>{
    next: Option<&'a mut Node<T>>
}
impl<'a, T> Iterator for MutIter<'a, T>{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}


impl<T> Drop for Stack<T> {
    fn drop(&mut self){
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}
