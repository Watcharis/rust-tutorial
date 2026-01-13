pub mod class_data_type_101 {
    use crate::tutorials::data_type_101::class_data_type_101;
    use std::{io, usize};

    pub fn main_data_type() {
        println!("start - fn main_data_type ....!!");

        class_data_type_101::scalar_types();

        class_data_type_101::compound_types();

        let index = class_data_type_101::input_data();
        // Using the index to find an element in an array
        class_data_type_101::find_element_from_input(index);

        println!("Index from input_data: {}", index);
        println!("end - fn main_data_type ....!!");
    }

    pub fn scalar_types() {
        println!("start - fn scalar_types ....!!");

        // Scalar types
        // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 
        // You may recognize these from other programming languages. Let’s jump into how they work in Rust.
        
        let a: i32 = 10; // Integer
        let b: f64 = 3.14; // Floating point
        let c: char = 'A'; // Character
        let d: bool = true; // Boolean

        println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
    }

    pub fn compound_types() {
        println!("start - fn compound_types ....!!");

        // Compound types
        // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
        // Tuples are fixed-size collections of values of different types, while arrays are fixed-size collections of values of the same type.

        // Tuple
        let tuple: (i32, f64, char) = (10, 2.5, 'b');
        println!("tuple: {:?}", tuple);
        println!("tuple first element: {}", tuple.0);
        println!("tuple second element: {}", tuple.1);
        println!("tuple third element: {}", tuple.2);

        // Array
        let array: [i32; 3] = [1, 2, 3];
        println!("array: {:?}", array);
    }

    pub fn input_data() -> usize {
        println!("start - fn input_data ....!!");

        // Input data
        // Rust provides a way to read input from the user using the `std::io` module.
        let mut input = String::new();
        println!("Please enter some text:");

        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input.trim());

        let index: usize = input
        .trim()
        .parse() // Parse the input to a usize
        .unwrap_or_else(|_| {
            println!("Invalid input, defaulting to 0");
            0 // Default value if parsing fails
        });
        // .expect("Index entered was not a number");
        println!("You entered index: {}", index);

        println!("end - fn input_data ....!!");
        return index;
    }

    pub fn find_element_from_input(index:usize) -> i32 {
        println!("start - fn find_element_from_input ....!!");

        // Example array to search
        let array: [i32; 5] = [10, 20, 30, 40, 50];

        let element = array.get(index).expect("Index out of bounds"); // This will panic if the index is out of bounds

        println!("Element found: {}", element);
        println!("end - fn find_element_from_input ....!!");

        *element
    }
}