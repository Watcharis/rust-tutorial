use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub mod class_smart_pointer_101 {
    pub fn main_smart_pointer() {
        println!("start - fn main_smart_pointer ....!!");
        let b = Box::new(5);
        println!("b = {}", b);
    }
}