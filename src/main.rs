#![allow(dead_code)]
mod tree;
mod bad_stack;
mod basic_stack;

fn main() {
    let mut list = bad_stack::List::new();
    let mut list2 = basic_stack::Stack::new();
    list2.push(3);
    list2.push(3);
    list2.push(3);
    list2.push(3);
    list2.push(3);
    list.push(2);
    list.push(3);

    println!("{:?}", list);
    println!("{:?}", list2);
}
