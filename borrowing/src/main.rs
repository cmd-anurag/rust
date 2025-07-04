fn main() {
    // instead of moving ownership around, borrow a value without becoming its owner

    let s1 = String::from("hello");

    let _length = calculate_length(&s1);
    // the reference is passed to the function without moving the ownership, however this is an immutable reference and 
    // trying to change it will cause an error.
    // use a mutable reference

    let mut s = String::from("Hello");

    change(&mut s);

    // mutable references have an important restriction, only a single reference can exist for a mutable variable.
    // in other words, rust enforces XOR immutablility. 
    // I can have either - Any number of immutable references OR Exactly one mutable reference. but not both at the same time.

    // if there has to be MM, IM, MI references, ensure they dont overlap.

    let r1 = &mut s;
    
    println!("{r1}");

    let r2 = &mut s;

    println!("{r2}");

    // this is fine as the usage of both references arent overlapping
    
    // let r4 = &mut s;
    // let r3 = &s;
    // println!("{r4}");

    // these cause the lifetime of mutable and immutable refernces to overlap hence the error.


    // Dangling References

    let _reference_to_nothing = dangle();

    

}

fn dangle() -> String {
    // this function tries to return a refence to a string which goes out of scope.
    let s = String::from("test");
    // &s

    // trying to do this results in a compile time error.
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // because the value goes out of scope and gets deleted when the function ends

    // so the better way would be to return the string itself which will move the ownership outside the function and hence no deallocation.

    s
}

// fn change(str : &String) {
//     str.push_str("this wont work");
// }

fn change(str: &mut String) {
    str.push_str(" World!");
}

fn calculate_length(str : &String) -> usize {
    str.len()
}