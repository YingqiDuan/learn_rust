use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    {
        //panic!("crash and burn");

        // let v = vec![1, 2, 3];
        // v[99];

        // RUST_BACKTRACE=1 cargo run
    }

    {
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("There was a problem opening the file: {:?}", error),
        };
    }

    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }

    {
        let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    {
        let greeting_file_result = File::open("hello.txt").unwrap();
        // thread 'main' panicked at src/main.rs:4:49:
        // called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
    }

    {
        let greeting_file_result = File::open("hello.txt").expect("Failed to open hello.txt");
    }

    {
        // let greeting_file_result = File::open("hello.txt")?;
        fn read_username_from_file() -> Result<(), Box<dyn Error>> {
            let f = File::open("hello.txt")?;
            Ok(())
        }
    }

    {
        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }
    }

    {
        use std::net::IpAddr;

        let home: IpAddr = "127.0.0.1"
            .parse()
            .expect("Hardcoded IP Address should be vaild");
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file_result = File::open("hello.txt");
    let mut username = String::new();
    username_file_result?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
