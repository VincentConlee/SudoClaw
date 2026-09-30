use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::solver::toroid;


type NodePtr<T> = Rc<RefCell<SphericalNode<T>>>;
type WeakNodePtr<T> = Weak<RefCell<SphericalNode<T>>>;

pub struct SphericalNode<T> {
    pub up: Option<WeakNodePtr<T>>,
    pub down: Option<NodePtr<T>>,
    pub left: Option<WeakNodePtr<T>>,
    pub right: Option<NodePtr<T>>,
    pub data: T,
}

impl<T> SphericalNode<T> {
    pub fn new(data: T) -> NodePtr<T> {
        let temp = Rc::new( RefCell::new( Self {up: None, down: None, left: None, right: None, data}));
        temp.borrow_mut().up = Some(Rc::downgrade(&temp));
        temp.borrow_mut().down = Some(temp.clone());
        temp.borrow_mut().left = Some(Rc::downgrade(&temp));
        temp.borrow_mut().right = Some(temp.clone());
        temp
    }
}

pub struct ToroidList<T> {
    pub head: Option<NodePtr<T>>
}

impl<T> ToroidList<T> {
    pub fn new(node: NodePtr<T>) -> Self {
        Self {head: Some(node.clone())}
    }

    pub fn insert_below(base: NodePtr<T>, node: NodePtr<T>) -> NodePtr<T> {
        let old_bottom = base.borrow().down.clone();
        base.borrow_mut().down = Some(node.clone());
        
        node.borrow_mut().up = Some(Rc::downgrade(&base));
        node.borrow_mut().down = old_bottom.clone();
        
        if let Some(node) = old_bottom {
            node.borrow_mut().up = Some(Rc::downgrade(&node));
        }

        node
    }

    pub fn insert_right(base: NodePtr<T>, node: NodePtr<T>) -> NodePtr<T> {
        let old_right = base.borrow().right.clone();
        base.borrow_mut().right = Some(node.clone());

        node.borrow_mut().left = Some(Rc::downgrade(&base));
        node.borrow_mut().right = old_right.clone();

        if let Some(node) = old_right {
            node.borrow_mut().left = Some(Rc::downgrade(&node));
        }

        node
    }

    pub fn insert_below_and_right(above_node: NodePtr<T>, left_node: NodePtr<T>, node: NodePtr<T>) -> NodePtr<T> {
        toroid::ToroidList::insert_right(left_node, node.clone());
        toroid::ToroidList::insert_below(above_node, node.clone());
        node
    }

}

impl<T> Drop for ToroidList<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        let mut down_one = curr.clone().unwrap().borrow_mut().down.take();
        while let Some(node) = curr.clone() {
            node.borrow_mut().down = None;
            curr = node.borrow().right.clone();
            node.borrow_mut().right = None;
        }

        while let Some(node) = down_one.clone() {
            node.borrow_mut().right = None;
            down_one = node.borrow().down.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    use super::*;
    
    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);
    struct LeakDetector;

    impl Drop for LeakDetector {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_toroid_list_leak() {
        DROP_COUNT.store(0, Ordering::SeqCst);

        {    
            let list = ToroidList::new(SphericalNode::new(LeakDetector));

            let mut right_seeking = list.head.clone().unwrap();
            let mut down_seeking = list.head.clone().unwrap();

            for _ in 1..4 {
                right_seeking = ToroidList::insert_right(right_seeking.clone(), SphericalNode::new(LeakDetector));    
                down_seeking = ToroidList::insert_below(down_seeking.clone(), SphericalNode::new(LeakDetector));
            }

            let mut down_base = list.head.clone().unwrap().borrow().down.clone().unwrap();
            let mut right_base = list.head.clone().unwrap();


            for _ in 0..3 {
                let mut interior_base = down_base.clone();
                for _ in 0..3 {
                    right_base = right_base.clone().borrow().right.clone().unwrap();
                    interior_base = ToroidList::insert_below_and_right(right_base.clone(), interior_base.clone(), SphericalNode::new(LeakDetector));
                }    
                down_base = down_base.clone().borrow().down.clone().unwrap();
                right_base = right_base.clone().borrow().down.clone().unwrap().borrow().right.clone().unwrap();
            }
        }

        assert_eq!(16, DROP_COUNT.load(Ordering::SeqCst))
    }

    #[test]
    fn test_toroid_construction() {
        let list = ToroidList::new(SphericalNode::new(1));

        let mut right_seeking = list.head.clone().unwrap();
        let mut down_seeking = list.head.clone().unwrap();

        for i in 1..4 {
            right_seeking = ToroidList::insert_right(right_seeking.clone(), SphericalNode::new(i+1));    
            down_seeking = ToroidList::insert_below(down_seeking.clone(), SphericalNode::new((4 * i) + 1));
        }

        let mut down_base = list.head.clone().unwrap().borrow().down.clone().unwrap();
        let mut right_base = list.head.clone().unwrap();
        let mut data = down_base.borrow().data;


        for _ in 0..3 {
            let mut interior_base = down_base.clone();
            for _ in 0..3 {
                right_base = right_base.clone().borrow().right.clone().unwrap();
                interior_base = ToroidList::insert_below_and_right(right_base.clone(), interior_base.clone(), SphericalNode::new(data + 1));
                data += 1;
            }    
            data += 1;
            down_base = down_base.clone().borrow().down.clone().unwrap();
            right_base = right_base.clone().borrow().down.clone().unwrap().borrow().right.clone().unwrap();
        }

        let mut curr = list.head.clone();

        let mut expected = 1;

        for _ in 0..4 {
            for _ in 0..4 {
                match curr {
                    Some(node) => {
                        assert_eq!(expected, node.borrow().data);
                        expected += 1;
                        curr = node.borrow().right.clone();
                    },
                    None => {panic!()}
                }
            }
            match curr {
                Some(node) => {
                    curr = node.borrow().down.clone();
                },
                None => {panic!()}
            }
        }
    }
}
