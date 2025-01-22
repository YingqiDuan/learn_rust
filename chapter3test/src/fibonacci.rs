use std::io;

pub(crate) fn main(){
    println!("Fibonacci");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Invalid input");
    let fib = fibonacci(n);
    println!("{}", fib);
}

fn fibonacci(n: u32) -> u64 {
    if n==0{
        0
    } else if n==1{
        1
    } else {
        let mut a = 0;
        let mut b = 1;
        let mut result = 0;

        for _ in 2..=n {
            result = a + b;
            a = b;
            b = result;
        }
        result
    }
}