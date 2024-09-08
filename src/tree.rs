#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Node{
    pub parent: Option<Rc<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub data: i32,
}

impl Node {
    pub fn new(data: i32) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Node {
                    data,
                    parent: None,
                    children: vec![],
                }
            )
        )
    }

    pub fn add_child(this: &Rc<RefCell<Self>>, data: i32) -> Rc<RefCell<Self>>{
        let new_tree = Rc::new(
            RefCell::new(
                Node{
                    parent: Some(Rc::clone(this)),
                    children: vec![],
                    data
                }
            )
        );
        this.borrow_mut().children.push(Rc::clone(&new_tree));
        new_tree
    }
    
    // Utility function to print the tree for debugging purposes
    pub fn print_tree(node: &Rc<RefCell<Node>>, depth: usize) {
        // Print the current node's data with indentation based on depth
        println!("{:indent$}Node data: {}", "", node.borrow().data, indent = depth * 2);

        // Recursively print all children nodes
        for child in &node.borrow().children {
            Node::print_tree(child, depth + 1);
        }
    }
}

