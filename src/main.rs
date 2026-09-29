use dlx_sudoku::solver::toroid::{SphericalNode, ToroidList};

fn main() {
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

    for _ in 0..8 {
        for _ in 0..24 {
            match curr {
                Some(node) => {
                    print!("{}, ", node.borrow().data);
                    curr = node.borrow().right.clone();
                },
                None => {panic!()}
            }
        }
        print!("\n");
        match curr {
            Some(node) => {
                curr = node.borrow().down.clone();
            },
            None => {panic!()}
        }
    }
}


// 1  2  3  4
// 5  6  7  8
// 9  10 11 12
// 13 14 15 16