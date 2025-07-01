
fn main() {
    println!("Hello world!");
    another_function(5);
    print_labeled_measurements(5, 'h');
    
    // Statements vs Expressions
    let _y = 6; // statement

    let _ = 5+6; // 5+6 is an expression

    // a new scope block is also an expression

    let z = {
        let x = 4;
        x+3 // no semicolons at the end of expressions
    };
    // this is somewhat equivalent to writing let z = x + 3 except x is defined right before assignment.

    println!("The value of z is {z}");

    let returned_value = five();
    println!("The returned value is {returned_value}");

    let num = 6;
    let num = plus_one(num);

    println!("The value after plus one is {num}");

}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}


fn five() -> i32 {
    // functions implicitly return the last expression of the body.
    5
}


fn plus_one(num: i32) -> i32 {
    num + 1
}

// if the function cannot find an expression to return it returns a unit () implicitly.