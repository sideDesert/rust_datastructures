
type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct Stack {
    head: Link,
}

#[derive(Debug)]
struct Node {
    data: i32,
    next: Link,
}


impl Stack {
    pub fn new() -> Self{
        Stack {
            head: None
        }
    }

    pub fn push(&mut self, data: i32){
        let new_node = Node {
            data,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(1)
            }
        }

    }
}
