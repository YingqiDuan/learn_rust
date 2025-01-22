use std::io;
fn main() {
    let mut x = 5;
    println!("{}", x);
    let x = x + 1;
    {
        let x = x * 2;
        println!("{}", x);
    }
    println!("{}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let spaces = "   ";
    let spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0;
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;

    let a = [1, 2, 3, 4, 5];
    let b = [3; 5];
    let first = a[0];
    let second = b[1];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");

    let element = a[index];
    println!("The value of the element at index {} is: {}", index, element);
}
