pub mod class_handle_error {
    use std::str;
    use std::fs::{File, OpenOptions};
    use std::io::{self, BufRead,ErrorKind, Error, Read, Write};
    use std::path::Path;
    
    pub fn main_error() {
        handle_file_error()
    }

    pub fn handle_file_error() {
        let path = Path::new("hello.txt");
        let display = path.display();
        println!("display: {:?}", display);
    
        // Open the path in read-only mode, returns `io::Result<File>`
        // let mut file = match File::open(&path) {
        //     // The `description` method of `io::Error` returns a string that describes the error
        //     Err(error) => match error.kind() {
        //         ErrorKind::NotFound => {
        //             println!("File not found: {}", display);

        //             match File::create(&path) {
        //                 Ok(file) => file,
        //                 Err(e) => panic!("Problem creating the file: {:?}", e),
        //             }
        //         }
        //         other_error => {
        //             panic!("Problem opening the file: {:?}", other_error)
        //         }
        //     },
        //     Ok(file) => {
        //         println!("File found: {}", display);
        //         file
        //     },
        // };

        // match file.write_all(b"Hello, world! 12345788") {
        //     Err(error) => {
        //         panic!("Problem writing to the file: {:?}", error)
        //     },
        //     Ok(file) => file,
        // }

        // // Read the file contents into a string, returns `io::Result<usize>`
        // let mut s = String::new();
        // match file.read_to_string(&mut s) {
        //     Err(why) => panic!("Problem reading the file: {:?}", why),
        //     Ok(_) => print!("{} contains:\n{}", display, s),
        // }

        let result_write_file = function_for_process_write_file();
        if let Some(error) = result_write_file.err() {
            panic!("Problem writing to the file: {:?}", error);
        }

        let result_read_file: Result<(), Error> = function_for_process_read_file();
        if let Some(error) = result_read_file.err() {
            panic!("Problem reading to the file: {:?}", error);
        }

        read_file_to_string();

        let response = read_username_from_file();
        if let Some(error) = response.err() {
            panic!("Problem reading the file: {:?}", error);
        }

        let result = read_username_from_file_2();
        if let Some(error) = result.err() {
            panic!("Problem reading the file: {:?}", error);
        }

        let result = last_char_of_first_line("hello world");
        if let Some(error) = result {
            println!("last_char_of_first_line: {:?}", error);
        }

    }

    fn function_for_process_write_file() -> io::Result<()> {
        let fs = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .open("hello_bug.txt");
        
        let mut file = match fs {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        for i in 0..10 {
            match file.write_fmt(format_args!("Hello, world! {}\n", i)) {
                Err(error) => {
                    print!("Problem writing to the file: {:?}", error);
                    return Err(error)
                },
                Ok(_) => {println!("write file success: {}", i); ()}
            };

            match file.flush().err() {
                Some(error) => {
                    println!("Problem flushing the file: {:?}", error);
                    return Err(error);
                },
                None => (),
            }
        }

        Ok(())
    }

    fn function_for_process_read_file() -> io::Result<()> {
        let fs = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open("hello.txt");

        let file = match fs {
            Ok(file) => file,
            Err(e) => {
                println!("Problem opening the file: {:?}", e);
                return Err(e).expect("[ ERROR - fn function_for_process_read_file ] : Problem opening the file failed");
            },
        };

        let readers = io::BufReader::new(file);
        for line in readers.lines() {
            match line {
                Err(e) => {
                    println!("Problem reading the file: {:?}", e);
                    return Err(e)
                },
                Ok(line) => println!("{}", line),
            }
        }

        Ok(())
    }

    fn read_file_to_string() {
        let path = Path::new("hello.txt");
        
        let open_file = File::open(&path);
        let mut file = match open_file {
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(&path) {
                    Ok(file) => file,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
            Ok(file) => file,
        };

        let mut buf_str= Vec::new();

        let result_read_file: Result<usize, Error> = file.read_to_end(&mut buf_str);
        match result_read_file {
            Err(error) => panic!("Problem reading the file: {:?}", error),
            Ok(res) => {
                println!("read file success: {:?}", res);
                println!("file content: {:?}", str::from_utf8(&buf_str).unwrap());
            }
        };

        
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e).expect("[ERROR] : Problem opening the file failed"),
        };

        let mut username = String::new();

        // match username_file.read_to_string(&mut username) {
        //     Ok(_) => {
        //         println!("username: {:?}", username);
        //         Ok(username)
        //     },
        //     Err(e) => Err(e),
        // }
        username_file.read_to_string(&mut username)?;
        println!("username: {:?}", username);
        Ok(username)
    }

    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut username = String::new();
    
        File::open("hello_xxx.txt")?.read_to_string(&mut username)?;
    
        Ok(username)
    }

    fn read_username_from_file_3() -> Result<File, io::Error> {
        let file: File = File::open("hello.txt")?;
        Ok(file)
    }

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}