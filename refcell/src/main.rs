use std::cell::RefCell;

fn main() {
    let a = RefCell::new(1);
    let x = a.borrow();
    let  y = a.borrow();
    let z = a.borrow_mut();
    // ERROR: 不可变和可变借用同时存在
}
