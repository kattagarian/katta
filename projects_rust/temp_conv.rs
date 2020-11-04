// Temperature conversor between fahrenheit and celsius
// This is part of Rust book to learn Rust programming language
// https://doc.rust-lang.org/book/
use std::io;
fn main () {
    loop {
        println!("Type the temperature: ");
        let mut temp = String::new(); //This create a string variable, idk yet how can i create directly an integer or float variable
        io::stdin() //this is responsible for take the input from user
            .read_line(&mut temp)
	    .expect("Failed");
        let temp: f64 = match temp.trim().parse() {// this verified if the input is a float and return Err if isnt and restart the loop. I use this because idk how to do it directly on the input function
            Ok(num) => num,
            Err(_) => continue,
        };
	let celsius: f64 = (5.0/9.0) * (temp - 32.0); //these calculates the temperature
	let fah: f64 = (9.0/5.0)*temp + 32.0;
        println!("Your temperature in C is: {}\nYour temperature in F is: {}", celsius, fah);//this show the results
    };
}
