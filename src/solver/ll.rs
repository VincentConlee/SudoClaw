use std::rc::{Rc, Weak};
use std::cell::{RefCell};

type NodePtr<T> = Rc<RefCell<CircularNode<T>>>;
type WeakNodePtr<T> = Weak<RefCell<CircularNode<T>>>;

pub struct CircularNode<T> {
    pub right: Option<NodePtr<T>>,
    pub left: Option<WeakNodePtr<T>>,
    pub data: T
}

impl<T> CircularNode<T> {
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        let curr = Rc::new(RefCell::new(Self{right: None, left: None, data: data}));
        curr.borrow_mut().right = Some(curr.clone());
        curr.borrow_mut().left = Some(Rc::downgrade(&curr));
        curr
    }
}

pub struct DoublyLinkedList<T> {
    pub head: Option<NodePtr<T>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new(node: NodePtr<T>) -> Self{
        Self { head: Some(node.clone()) }
    }

    pub fn insert_right(base: NodePtr<T>, new_node: NodePtr<T>) {
        let old_right = base.borrow_mut().right.clone();
        base.borrow_mut().right = Some(new_node.clone());
        new_node.borrow_mut().right = old_right.clone();
        new_node.borrow_mut().left = Some(Rc::downgrade(&base));

        if let Some(node) = old_right {
            node.borrow_mut().left = Some(Rc::downgrade(&new_node));
        }
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        if let Some(node) = self.head.take() {
            node.borrow_mut().right = None
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
    fn test_circular_list_leak() {
        DROP_COUNT.store(0, Ordering::SeqCst);

        {    
            let list = DoublyLinkedList::new(CircularNode::new(LeakDetector));
            DoublyLinkedList::insert_right(list.head.clone().unwrap(), CircularNode::new(LeakDetector));
            DoublyLinkedList::insert_right(list.head.clone().unwrap(), CircularNode::new(LeakDetector));
            DoublyLinkedList::insert_right(list.head.clone().unwrap(), CircularNode::new(LeakDetector));
            DoublyLinkedList::insert_right(list.head.clone().unwrap(), CircularNode::new(LeakDetector));
            DoublyLinkedList::insert_right(list.head.clone().unwrap(), CircularNode::new(LeakDetector));
        }

        assert_eq!(6, DROP_COUNT.load(Ordering::SeqCst))
    }
}
