pub mod class_hashmap {
    use std::collections::HashMap;
    use crate::tutorials::hashmap_101::updating_value_based_on_old_value;

    pub fn main_hashmap(){

        let mut scores: HashMap<String, i32> = HashMap::new();

        // insert key and value into hashmap
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        scores.insert( String::from("Black"), 0);
        scores.insert(String::from("Blue"), 100);

        println!("scores: {:?}", scores);

        // get value from key
        let result = scores.get(&String::from("Red")).unwrap_or_else(|| -> &i32 { 
            let default_value :&i32 = &300;
            default_value
        });
        println!("Red: {:?}", result);

        // get value from key use defalut value copy
        let result = scores.get(&String::from("Yellow")).copied().unwrap_or(300);
        println!("Red: {:?}", result);

        // get value and key
        let result =  scores.get_key_value(&String::from("Blue")).unwrap();
        println!("keys: {:?}, value: {:?}", result.0, result.1);

        // check key is exists
       let red_key = String::from("Red");
        if if_not_exists(&mut scores, &red_key) {
            println!("Red key is not exists");
            add_to_hashmap(&mut scores, red_key, 300);
        }

        // loop hashmap
        // let new_scores = scores.clone();
        // println!("scores: {:?}", new_scores);
        loop_hashmap(&mut scores);


        // --------------------------------
        updating_value_based_on_old_value();

    } 

    pub fn if_not_exists(destinies: &mut HashMap<String, i32>, key: &String) -> bool {
        println!("if_not_exists hashmap.....!!!\n");
        let res = destinies.get(key);
        println!("res: {:?}", res);
        if res == None {
            return true;
        }else {
            return false;
        }
    }  

    pub fn add_to_hashmap(destinies: &mut HashMap<String, i32>, key: String, value: i32) {
        println!("add_to_hashmap .....!!!\n");
        destinies.insert(key, value);
    }

    pub fn loop_hashmap(destinies: &mut HashMap<String, i32>) {
        println!("loop_hashmap .....!!!\n");
        for (key, value) in destinies {
            println!("key: {}, value: {}", key, value);
        }
    }
}


// Updating a Value Based on the Old Value
fn updating_value_based_on_old_value() {
    use std::collections::HashMap;
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        print!("count: {:?}, ", count);
        // *count คือ การอ้างอิงไปถึงค่าของ count และเพิ่มค่าของ count อีก 1 
        // เนื่องจาก count เป็น & เป็นการอ้างอิงถึงค่าของ count จึงต้องใช้ * นำหน้า
        *count += 1;
    }

    println!("{map:?}");
}