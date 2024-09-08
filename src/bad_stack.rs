type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct List {
    head: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List{head: Link::None}
    }

    pub fn push(&mut self, elem: i32){
        let new_node = Box::new(Node {
            elem,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<i32>{
        match self.head.take(){
            None => None,
            Some(node) => {
                let result = Some(node.elem);
                self.head = node.next;
                result
            }
        }
    }
}


