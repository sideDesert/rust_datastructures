use std::sync::Arc;

type Link<T> = Option<Arc<Node<T>>>;

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
    pub fn new()-> Self{
        Stack {
            head: Link::None
        }
    }

    pub fn prepend(&mut self, data: T) -> Self{
        let new_node= match self.head.as_ref(){
            None => {
                Node {
                    data,
                    next: Link::None
                }
            }
            Some(self_head) => {
                Node {
                    data,
                    next: Link::Some(Arc::clone(self_head))
                }
            }
        };

        Stack {
            head: Some(Arc::new(new_node))
        }
    }

    pub fn tail(&self)-> Self{
        Stack {
            head: self.head.as_ref().map(Arc::clone)
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter{next: self.head.as_deref()}
    }
}


// for the iterators
pub struct Iter<'a,T> {
    next: Option<&'a Node<T>>
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

