#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

type RefCountNode = Rc<Node>;

#[derive(Debug)]
pub struct Node {
    value: i32,
    parent: RefCell<Option<RefCountNode>>,
    children: RefCell<Vec<RefCountNode>>,
}

pub fn test_circular_ref() {
    let child = Rc::new(Node {
        value: 10,
        parent: RefCell::new(None),
        children: RefCell::new(vec![]),
    });

    let parent = Rc::new(Node {
        value: 1,
        parent: RefCell::new(None),
        children: RefCell::new(vec![Rc::clone(&child)]),
    });

    // program crashes when we enable the following line
    *child.parent.borrow_mut() = Some(Rc::clone(&parent));

    println!("Parent: {:?}", parent);
    println!("Children: {:?}", child)
}
