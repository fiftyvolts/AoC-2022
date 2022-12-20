
use std::{env::var, fs::read_to_string, rc::{Rc}, cell::RefCell};

// lazy_static! {
//     static ref INPUT: String = read_to_string(var("INPUT")).unwrap()).unwrap();
// }

#[derive(Debug, Clone)]
struct Indirect {
    pos: usize,
    data: i32,
    next: Option<Rc<Indirect>>,
    prev: Option<Rc<Indirect>>
}


fn main() {
    let nums = read_to_string(var("INPUT").unwrap())
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i,n)| (i, i32::from_str_radix(n, 10).unwrap()))
        .collect::<Vec<_>>();
    
    let head = Rc::new(Indirect {
        pos: nums[0].0,
        data: nums[0].1,
        next: None,
        prev: None
    });

    let mut last = head.clone();

    for i in 1..nums.len() 
        Rc::get_mut(&mut last).unwrap().next = Some(Rc::new(Indirect {
            pos: nums[i].0,
            data: nums[0].1,
            next: None,
            prev: Some(last.clone())
        }));
        last = last.next.as_ref().unwrap().clone(); 
    }

    let mut curr = Some(head.clone());
    while curr.is_some() {
        println!("{} {}", curr.as_ref().unwrap().pos, curr.as_ref().unwrap().data);
        curr = curr.as_ref().unwrap().next.clone();
    }

}
