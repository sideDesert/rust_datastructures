type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    next: Link<T>,
    data: T
}

#[derive(Debug)]
pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>
}


impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, data: T){
        let mut new_tail = Box::new(Node{
            data,
            next: None
        });

        let new_tail_ptr: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe{
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = new_tail_ptr;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head|{
            self.head = old_head.next;
            if self.head.is_none(){
                self.tail = std::ptr::null_mut();
            }
            old_head.data
        })
    } 
}



#[cfg(test)]
mod test {
    use crate::queue::Queue;
    #[test]
    fn basics() {
        let mut q = Queue::new();

        // Check empty list behaves right
        assert_eq!(q.pop(), None);

        // Populate list
        q.push(1);
        q.push(2);
        q.push(3);

        // Check normal removal
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        q.push(4);
        q.push(5);

        // Check normal removal
        assert_eq!(q.pop(), Some(3));
        assert_eq!(q.pop(), Some(4));

        // Check exhaustion
        assert_eq!(q.pop(), Some(5));
        assert_eq!(q.pop(), None);

        // Check the exhaustion case fixed the pointer right
        q.push(6);
        q.push(7);

        // Check normal removal
        assert_eq!(q.pop(), Some(6));
        assert_eq!(q.pop(), Some(7));
        assert_eq!(q.pop(), None);
    }
}
