#![allow(dead_code)]
mod tree;
mod stack;
mod persistent_stack;
mod persistent_atomic_stack;
mod deque;

#[derive(Debug)]
struct Foo {
    val: i32
}

fn main() {
    let list1 = persistent_atomic_stack::Stack::new().prepend(32).prepend(21);
    let list2 = persistent_atomic_stack::Stack::new().prepend(list1.tail());
    
    println!("{:?}", list2);
    println!("{:?}", list1);
}
