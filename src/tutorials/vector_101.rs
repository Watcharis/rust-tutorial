pub mod class_vector {
    use std::any::Any;
    use crate::tutorials::vector_101::class_vector;
    use crate::vector_101;

    const VECTOR_SIZE: i32 = 10;

    // #[derive(Debug)]
    #[derive(Debug, PartialEq, Clone)]
    // Copy ไม่สามารถใช้ได้เนื่องจาก struct มี field เป็น String ซึ่ง String ไม่สามารถใช้ copy ได้ #[derive(Copy)]
    struct SimpleSturct {
        name : String,
        mobile: String,
    }


    pub fn main_vector() {

        println!("start call vector 101 !!!");

        // vector สามารถนำมาใช้ในรูปแบบ genneric type ได้ 
        // Example : Vec<T> โดย T สามารถเป็น type ใดๆก็ได้
        let mut v : Vec<i32> = Vec::new();
        v.push(234);
        println!("v : {:?}", v);

        let mut v_genric_type: Vec<Box<dyn Any>> = Vec::new();

        v_genric_type.push(Box::new(1));

        v_genric_type.push(Box::new(String::from("bob")));

        v_genric_type.push(Box::new(class_vector::SimpleSturct{
            name: String::from("bom"),
            mobile: String::from("0654478898"),
        }));
        
        println!("v_genric_type : {:?}", v_genric_type);

        for (index, item) in v_genric_type.iter().enumerate(){
            println!("index: {}", index);

            if let Some(value) = item.downcast_ref::<i32>() {
                println!("Integer: {}", value);
            }else if let Some(value) = item.downcast_ref::<String>() {
                println!("String: {}", value);
            }else if let Some(value) = item.downcast_ref::<class_vector::SimpleSturct>() {
                println!("struct class_vector::SimpleSturct: {:#?}", value);
                println!("struct class_vector::SimpleSturct field: {:?}, {:?}", value.name, value.mobile);
            }
        }

        class_vector::vector_loop();

        class_vector::reading_element_vector();
    }

    pub fn vector_loop() {
        let mut simple_data_store: Vec<SimpleSturct> = Vec::new();
        let mut count: i32 = 0;
        loop {
            if count >= VECTOR_SIZE {
                break println!("count: {count}");
            }

            let username: String = format!("{}_{}",  "user", count);
            let mobile_number = vector_101::gennerate_mobile_phone();
            let data :SimpleSturct = SimpleSturct{
                name: username,
                mobile: mobile_number,
            };
            simple_data_store.push(data);

            count += 1
        }
        println!("simple_data_store : {}", simple_data_store.len());


        // iter() กับ into_iter() แตกต่างกันอย่างไร
        // - iter() : จเป็นการยืมค่าทีละตัวจาก Vec
        // - into_iter : เอาค่าออกจาก Vec แล้วคืน Vec ให้ Rust จัดการ drop

        // for (i, _) in simple_data_store.iter().enumerate() {
        // println!("index {}, value : {:?}", i, simple_data_store.get(i));
        // }

        for (i, value) in simple_data_store.into_iter().enumerate() {
            println!("index {}, value : {:?}", i, value);
        }
    }

    pub fn reading_element_vector() {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {third}");
    
        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }
}

use rand::Rng;
pub fn gennerate_mobile_phone() -> String {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let prefix: String = (0..3).map(|x| {
        if x == 0 {
            String::from("0")
        }else {
            // let random_number = rng.gen_range(6..10);
            if  x == 1 {
                let random_number = rng.gen_range(6..10);
                if random_number == 7 {
                    return rng.gen_range(8..10).to_string();
                }
                return random_number.to_string();

            }else {
                rng.gen_range(6..10).to_string()
            }
        }
    }).collect();
    // println!("prefix : {prefix}");

    let number: String = (0..7).map(|_| rng.gen_range(0..10).to_string()).collect();
    // println!("number = {number}");
    
    format!("{}{}", prefix, number)
}












