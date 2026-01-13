#[derive(Debug)]
pub enum FlowersVariant {
    Rose(FlowerTemp),
    Sunflower(FlowerTemp),
    AnyFlower(FlowerTemp),
}

#[derive(Debug)]
pub struct FlowerTemp {
    pub name: String,
    pub color: String,
    pub height: i32,
}

#[derive(Debug)]
pub struct Flowers {
    pub info: FlowersVariant,
}

pub trait FlowerService  {
    fn get_flower(&self, name: String) -> FlowersVariant;
}

impl FlowerService for Flowers {
    fn get_flower(&self, name: String) -> FlowersVariant {
        if name == "rose" {
            let rose = FlowersVariant::Rose(FlowerTemp{
                name,
                color: String::from("red"),
                height: 122,
            });
            dbg!(&rose);
            return  rose;
        }else if name == "sunflower" {
            let sunflower = FlowersVariant::Sunflower(FlowerTemp{
                name: String::from("sunflower"),
                color: String::from("yellow"),
                height: 544,
            });
            dbg!(&sunflower);
            sunflower
        }else {
            FlowersVariant::AnyFlower(FlowerTemp{
                name: String::from("ivy"),
                color: String::from("green"),
                height: 326,
            })
        }
    }
}

pub mod classflower {
    use crate::garden::flower::Flowers;
    use crate::garden::flower::FlowerTemp;
    use crate::garden::flower::FlowersVariant;

    pub fn new_flower_object(name: String, color : String, height: i32) -> Flowers {
        let init_folwers: Flowers = Flowers{
            info: FlowersVariant::AnyFlower(FlowerTemp{
                name,
                color,
                height,
            }),
        };

        init_folwers
    }
}

mod front_of_flower {
    pub mod hosting {
        pub fn add_to_front_flower() -> bool {
            true
        }
    }
}

// use crate::front_of_flower::hosting::add_to_front_flower;

pub fn add_flower() {
    front_of_flower::hosting::add_to_front_flower();
}

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         pub seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }