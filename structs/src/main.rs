struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, structs!");

    let user = User {
        active: true,
        username: String::from("anurag"),
        email: String::from("anurag@example.com"),
        sign_in_count: 1,
    };

    let email = user.email;
    let _active = user.active;
    let _sign_in_count = user.sign_in_count;
    let _username = user.username;

    println!("{email}");

    let mut muser = User {
        active: true,
        username: String::from("anurag"),
        email: String::from("anurag@example.com"),
        sign_in_count: 1,
    }; // a mutable instance of User struct

    // making only specific fields mutable is not allowed.

    muser.email = String::from("changedemail@example.com");
    let changed_email = muser.email;
    println!("{changed_email}");


    // if there is a need to create a new instance using the values of an existing instance (or by changine only few of them), use the struct update syntax

    let user2 = User {
        username: String::from("johndoe"),
        email: String::from("johndoe@example.com"),
        ..user
    };

    // username and email will be set accordingly and rest of the fields will be fetched from user instance


    let _user3 = User {
        username: String::from("newuser"),
        ..user2
    };

    // this causes user2.email to MOVE to user3 as one would expect when normally assigning String variables to another variable.

    // let user2_email = user2.email; ;
    // doing this will cause an error

    // Tuple Structs

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // even though they both have three same typed elements, they have different types

    // it is possible to access individual values using . and destructure a tuple struct like a regular tuple .
    // however naming the struct is required when destructuring them

    // instead of 
    // let (x, y, z) = origin;

    let Point(_x, _y, _z) = origin; // correct way
    let Color(_r, _g, _b) = black;

    // unit struct instance
    let _test = AlwaysEqual;

}

fn _build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,  // field init shorthand since parameter name and field name is same
        sign_in_count: 1,
    }
}

// Tuple structs - doesnt have named field, instead only types are mentioned.
// useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit Structs - no fields 
// mainly used to implement a trait on a type but doesnt require to store any data  

struct AlwaysEqual;