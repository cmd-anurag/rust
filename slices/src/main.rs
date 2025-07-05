fn main() {
    // SLICES - a reference to a contiguous sequence of elements of a collection.

    // a small programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
    

    let mut s = String::from("Hello World");
    let word = first_word(&s);

    s.clear();

    println!("{word}");


    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}");
    println!("{world}");

    // these are references to part of the string , and it directly points to the start of the string's bytes on the heap, along with its length.
    // in contrast to regular references which point to the struct on the stack which then points to the actual heap content..

    let first_word_result = improved_first_word(&s);

    // s.clear();
    // now trying to do this will result in an error as both an immutable reference returned from the function and the immutable reference needed by clear() cannot coexist.


    println!("{first_word_result}");

    // array slices exist too
    let arr = [1, 2, 3, 4, 5];

    let _slice1 = &arr[1..3];
    let _slice2 = &arr[3..];

}

fn first_word(s: &String) -> usize {

    // convert the string to an array of bytes to check each element
    let bytes = s.as_bytes();

    // iter is used to return each element in a collection (in this case a byte) and enumerate changes each element into a tuple of the index and the reference to the element.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn improved_first_word(s: &String) -> &str { // small improvement here could be changing &String to &str in the paramater.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}