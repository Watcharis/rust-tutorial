pub mod class_lifetime_101 {

    pub fn main_lifetime() {
        println!("start - fn main_lifetime ....!!");

        let string1 = String::from("long string is long");
        let string2 = String::from("xyz");

        let result =longest(string1.as_str(), string2.as_str());
        print!("The longest string is {}\n", result);

        let result_sc = longest_sc(string1, string2);
        print!("The longest string is {}\n", result_sc);


        let query = "duct".to_string();
        {
            let contents = "contents duct\nduct time\nduct jams".to_string();
            let result_search = search(query.as_str(), contents.as_str());
            print!("The search result is {:?}\n", result_search);
        }
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // func ไม่ได้มี การรับ parameter แบบ reference แต่รับแบบ value
    // เลย ไม่จำเป็นต้องใช้ lifetime ในการระบุ ว่า ตัวแปร x และ y มี lifetime เท่ากัน
    // เพราะ liftetime จะถูกใช้เมื่อมีการระบุ reference ใน parameter จากการใช้ & หรือ &mut (borrowing)
    fn longest_sc(x:String, y: String) -> String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| {
                println!("line: {:?}", line);
                line.contains(query)
            })
            .collect()
    }

}