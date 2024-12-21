use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value: value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new())
        }
    }
}

pub fn add_child(node: &Rc<Node>, child: Rc<Node>) {
    *child.parent.borrow_mut() = Rc::downgrade(node);
    node.children.borrow_mut().push(child);
} 

fn dump(node: &Rc<Node>) {
    print!("node({}) ", node.value);
    match node.parent.borrow().upgrade() {
        Some(parent) => {
            print!("parent..{}", parent.value);
        },
        None => {
            print!("parent..None");
        }
    }
    print!(" children..[");
    for child in &*node.children.borrow() {
        print!("{} ", child.value);
    }
    println!("]");

    for child in &*node.children.borrow() {
        dump(child);
    }
}

fn main() {
    let root = Rc::new(Node::new(0));

    let node1 = Rc::new(Node::new(1));
    *node1.parent.borrow_mut() = Rc::downgrade(&root);

    let node2 = Rc::new(Node::new(2));
    add_child(&node1, node2);

    let node3 = Rc::new(Node::new(3));
    add_child(&node1, node3);
    add_child(&root, node1);

    let node4 = Rc::new(Node::new(4));
    add_child(&root, node4.clone());

    let node5 = Rc::new(Node::new(5));
    add_child(&node4, node5);

    dump(&root);
}
