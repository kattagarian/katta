// Program to generate the nth fibonacci number
// This is part of Rust book to learn Rust programming language
// https://doc.rust-lang.org/book/

use std::io;
fn main() {
    loop {
    println!("This program generate the nth fibonacci number\nEnter the n:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed");
    let n: i64 = match n.trim().parse() {
	Ok(num) => num,
	Err(_) => continue,
    };
    let mut fib_num: i64 = 0;
    let mut fib_num0: i64 = 0;
    let mut fib_num1: i64 = 1;
    let mut index: i64 = 3;
    if n == 1 {
	    println!("The The {}th fibonacci number is {}", n, fib_num0);
	    continue;
    } else if n == 2{
	    println!("The {}th fibonacci number is {}", n, fib_num1);
	    continue;
    }	

    while index <= n {
        fib_num = fib_num0 + fib_num1;
	fib_num0 = fib_num1;
	fib_num1 = fib_num;
        index += 1;
    } 	
    println!("The {}th fibonacci number is {}", n, fib_num);
    }
}
