// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() -> bool { true }
//     }
// }


// use crate::garden::front_flower::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     // front_of_house::hosting::add_to_waitlist();
//     add_to_waitlist();
// }


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> bool { true }
    }
}

pub use crate::garden::front_flower::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}