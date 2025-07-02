fn main() {

    // ownership is interesting...

    let x = 10;
    let _y = x;

    // nothing fancy till now, since the size of these types are known and they are stored on stack, making a copy is trivial.
    // x's value 10 is copied to y. 

    
    // however things get interesting when i try this on a type stored on the heap.
    
    let str = String::from("Hello World");
    let _str2 = str;
    
    // this DOES NOT copy the actual string stored on heap into str2, instead the variable str which has a pointer to heap memory, a size
    // and a capacity is copied since str itself is on the stack.
    
    // when the owner of a value goes out of scope the value's memory is freed.

    // this introduces a bunch of problems , usually when str and str2 go out of scope they will try to free the same heap memory 
    // twice resulting in undefined behavior, 
    // rust instead moves the ownership of the variable to str2, rendering str useless and invalid, in essence the value has moved from str to str2;

    // println!("{str}");
    // trying to use str after it has moved will result in a compile time error.


    // in the inverse case when a new value is assigned to an already existing variable
    let mut _str3 = String::from("galaxy");
    _str3 = String::from("New String");
    println!("{_str3}");

    // the old value is dropped immediately and "New String" is printed.


    // now in case of function calls it behaves similary to assignment operation

    // passing this variable (that do not implement the copy trait) to a function moves the ownership to function variables
    // returning from a function returns the ownership

    ownership();
    returns();

}

fn ownership() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn returns() {
    let _s1 = gives_ownership();        // gives_ownership moves its return value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

// yeah... as one can painfully realize, moving values to and fro between functions to use them is annoyinng, luckliy there is the concept of borrowing.