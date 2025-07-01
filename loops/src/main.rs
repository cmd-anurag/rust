fn main() {
    loop {
        println!("again");
        break;
    }

    // a loop construct is equivalent to while true in most languages.
    // it can be used to retry operations until they succeed , and so it is possible to return a result of that operation out of the loop
    let mut counter = 0;

    let result = loop {

        counter += 1;

        if counter == 10 {
            break counter; // expression after the break keyword is returned.
        }
    };

    println!("The operation completed in {result}th attempt");

    // loop labels are a nice thing to have

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while loops
    let mut x = 0;
    while x < 10 {
        println!("The value of x is {x}");
        x += 1;
    }

    // for loops

    let arr = [1, 2, 3, 4, 5];

    for element in arr {
        println!("The element is {element}");
    }

    for i in 1..5 {
        print!("{i} ");
    }
    for i in (1..5).rev() {
        print!("{i} ");
    }
}
