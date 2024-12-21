use std::io;

fn main() {
    // Mutability
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // Shadowing
    // let x = 5;
    
    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in inner scope is: {x}");
    // }

    // println!("The value of x is: {x}");

    // Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
