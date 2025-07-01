use std::io;

fn main() {
    let mut x: u8 = 255;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let _f = 3.141592;
    let _b = false;
    let _c = 'a';

    // Tuples (fixed length, different types)

    let tuple = (_f, _b, _c, x);
    let third = tuple.2;
    println!("{third}");

    let _unit = ();
    
    // Arrays (fixed length, same types)

    let _arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let _arr2 = [4; 6];

    // let element = _arr[11];

    // println!("{element}"); // error, this operation will panic at runtime.

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = _arr[index];
    println!("The value of array at {index} is {element}");

}
