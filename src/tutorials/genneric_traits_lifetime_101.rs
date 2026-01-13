#[derive(Debug, PartialEq, Clone)]
struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T: std::fmt::Debug, U: std::fmt::Debug> Point<T, U> {
impl <T, U >Point<T, U> {
    pub fn display_value(&self) where T: std::fmt::Debug, U: std::fmt::Debug {
        println!("point x : {:?}", &self.x);
        println!("point y : {:?}", &self.y);
        println!("point : {:?}", &self);
    }

    pub fn get_x(&self) -> &T {
        &self.x
    }

    pub fn get_y(&self) -> &U {
        &self.y
    }

    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> where T: std::fmt::Debug + std::ops::Add, W: std::fmt::Debug {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// where ใน rust จะใช้ในการระบุ constraint ของ type ที่เป็น generic โดยใช้เงื่อนไขต่างๆ
// หรือ สามารถบอก ว่า type ที่เป็น generic ต้อง implement ตาม trait ที่กำหนด
impl<T: std::ops::AddAssign + std::fmt::Debug, U: std::ops::AddAssign + std::fmt::Debug> Point<T, U>{
    // case ที่ 1 ไม่ต้องใช้ where เพราะ trait ที่ใช้มีการ implement โดยใส่เงื่อนไขในการระบุ type ที่เป็น generic อยู่แล้ว
    // ซึ่งในที่นี้คือ std::ops::AddAssign + std::fmt::Debug
    pub fn add_x(&mut self, value: T) {
        print!("add_x : {:?}", value);
        self.x += value;
    }

    // case ที่ 2 ใช้เงื่อนไขในการระบุ type ที่เป็น generic โดยใช้ where
    pub fn add_y(&mut self, value: U) where U: std::ops::AddAssign + std::fmt::Debug {
        println!("add_y : {:?}", value);
        self.y += value;
    }
}

pub mod class_genneric_traits_lifetime {

    use crate::tutorials::genneric_traits_lifetime_101::Point;

    pub fn main_genneric_traits_lifetime() {
        println!("start - fn main_genneric_traits_lifetime ....!!");

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");
    
        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {result}");

        let mut init_point = Point{x: 5, y: 4.0 };
        init_point.display_value();

        let result_get_x = init_point.get_x();
        println!("result_get_x : {:?}", result_get_x);

        let result_get_y = init_point.get_y();
        println!("result_get_y : {:?}", result_get_y);

        // test stadard trait AddAssign
        init_point.add_x(5);
        init_point.add_x(String::from(" test")); // error เพราะ type ของ x เป็น i32 ไม่สามารถบวกกับ String ได้

        init_point.add_y(5.0);
        init_point.display_value();


        // method definition with more than one generic type parameter
        let init_point_2 = Point{x: "lat", y: "long" };
        let result_mix_up = init_point.mixup(init_point_2);
        println!("result_mix_up : {:?}", result_mix_up);
    }

    fn largest<'a, T: std::cmp::PartialOrd>(list: &'a [T]) -> &'a T {
        let mut largest = &list[0];
    
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
}


// fn print_and_clone<T>(item: T) where T: Clone + std::fmt::Debug{
//     println!("{:?}", item);
//     let _cloned = item.clone();
// }