#![allow(dead_code)]

mod tree;
mod stack;
mod persistent_stack;
mod persistent_atomic_stack;
mod deque;
mod queue;

#[derive(Debug)]
struct Foo {
    val: i32
}

struct MyStruct {
    head: Option<Box<i32>>,
    tail: Option<Box<i32>>
}

impl MyStruct {
    pub fn test(&mut self) {
        let _a = self.tail.as_deref_mut();
        let _b = self.head.as_deref_mut();
    }
}
fn main() {
    let mut my_struct = MyStruct{
        head: Some(Box::new(2)),
        tail: Some(Box::new(3)),
    };
    my_struct.test();
}
