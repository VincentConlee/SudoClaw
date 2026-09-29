// use std::iter::Enumerate;
// use std::ops::Deref;
// use std::rc::Rc;
// use std::cell::RefCell;

// type NodeLink = Rc<RefCell<Node>>;

// #[derive(Clone, Debug)]
// pub struct Node {
    // pub up: Option<NodeLink>,
    // pub down: Option<NodeLink>,
    // pub left: Option<NodeLink>,
    // pub right: Option<NodeLink>,
    // pub column: Option<NodeLink>,
    // pub size: usize,
    // pub name: usize,
// }

// impl Node {
    // pub fn new(col: Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Self>> {
        // Rc::new(RefCell::new( Node {
            // up: None,
            // down: None,
            // left: None,
            // right: None,
            // column: col,
            // size: 0,
            // name: 0,
        // }))
        
    // }

    // pub fn insert_above(&mut self, new: &NodeLink) {
        // self.up = Some(Rc::clone(new));
    // }

    // pub fn insert_below(&mut self, new: &NodeLink) {
        // self.down = Some(Rc::clone(new));
    // }

    // pub fn insert_left(&mut self, new: &NodeLink) {
        // self.left = Some(Rc::clone(new));
    // }

    // pub fn insert_right(&mut self, new: &NodeLink) {
        // self.right = Some(Rc::clone(new));
    // }
// }

// #[derive(Debug)]
// pub struct QuadList {
    // pub head: RefCell<Node>,
// }

// impl QuadList {
    // pub fn new() -> Self {
        // Self {
            // head: RefCell::new(Node::new()),
        // }
    // }

    // pub fn create_header_row(&mut self, sudoku_array: Vec<Vec<u8>>) {
        // let mut curr = self.head.borrow();
        // for (i, _) in sudoku_array.iter().enumerate() {
            // let new = RefCell::new(Node::new_with_id(i));
            // let mut temp = new.borrow_mut();
            // curr.insert_right(&Rc::new(new));
            // temp.insert_left(&Rc::new(curr));
        // }
    // }
// }