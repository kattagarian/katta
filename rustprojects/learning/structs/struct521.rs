/* This function is one of the ways to write this code. We can make it simpler
using tuples (above)
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area (width: u32, height: u32) -> u32 {
    width * height
}
*/

/* This function uses tuples to solve this problem. But tuples has a problem: lack of 
 * information. We will solve this in the next function
fn main () {
    let rect1 = (30,50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    /*Testing debug feature
    println!("rect1 is {:#?}", rect1);*/
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
