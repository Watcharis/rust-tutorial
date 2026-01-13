pub mod garden;
pub mod tutorials;
use garden::flower::FlowerService;

use crate::garden::vegetables;
// use crate::garden::vegetables::Asparagus;
use crate::garden::flower;
use crate::garden::front_flower;

use crate::tutorials::vector_101;
use crate::tutorials::hashmap_101;
use crate::tutorials::error_101;
use crate::tutorials::genneric_traits_lifetime_101;
use crate::tutorials::lifetime_101;
use crate::tutorials::smart_pointer_101;
use crate::tutorials::data_type_101;
// use uuid::Builder;
use uuid::{uuid, Uuid};

fn main() {
    println!("Hello, world!");
    println!("Start rust programing 101 !!!");

    // let text_start : String = String::from("start rust 101");
    // dbg!(&text_start);

    // let plant = vegetables::Asparagus{
    //     color: String::from("yellow"),
    //     size: 35,
    // };
    // println!("I'm growing {plant:?}!");

    // let newflower = flower::classflower::new_flower_object(
    //     String::from("ivy"), 
    //     String::from("green"), 
    //     326);
        
    // let flower_value = newflower.get_flower(String::from("gog"));
    // dbg!(&flower_value);

    // let add_wl = front_flower::hosting::add_to_waitlist();
    // dbg!(&add_wl);

    // let uuid_temp = Uuid::new_v4();
    // println!("uuid : {:?}", &uuid_temp);

    // let uuid_temp_str = uuid_temp.to_string();
    // println!("uuid_temp_str : {:?}", &uuid_temp_str);

    // const ID: Uuid = uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
    // println!("ID : {:?}", &ID);


    // data_type_101::class_data_type_101::main_data_type();

    vector_101::class_vector::main_vector();

    // hashmap_101::class_hashmap::main_hashmap();   

    // error_101::class_handle_error::main_error();

    // genneric_traits_lifetime_101::class_genneric_traits_lifetime::main_genneric_traits_lifetime();

    // lifetime_101::class_lifetime_101::main_lifetime();

    // smart_pointer_101::class_smart_pointer_101::main_smart_pointer();
}