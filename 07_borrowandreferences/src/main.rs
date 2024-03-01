// Borrowing in Rust
// Borrowing rust means temporarily access of value without ownership of it
// we cannot have mutable and immutable references point the same value at a time
// we can have one mutable reference and any number of immutable reference to a variable at the same time
// we cannot have mutable and immutable reference at the same time

fn main() {
    // string
    let my_string = String::from("Hello world");

    //  mutable reference
    let my_reference = &my_string;

    println!("My reference is {}", my_reference);

    let my_string = String::from("Hello world");

    print_string(&my_string);

    println!("{}", my_string);

    // mutable string
    let mut my_string = String::from("Hammad Ali");

    // mutable reference
    print_mut_string(&mut my_string);

    println!("{}", my_string);

    // Dangling reference example
    let new_string = String::from("Hello Rust");
    let _new_string_reference = return_reference(&new_string);

    // transfering ownership
    let _newer_string = new_string;

    // here we will get  error due to variable which we are referencing has been eleminated in memory
    //  println!("{}",_new_string_reference);
}

// functoin to print immutable reference string
fn print_string(s: &String) {
    println!("{}", s);
}
// functoin to print mutable reference string
fn print_mut_string(s: &mut String) {
    s.push_str(" Khan");
}

fn return_reference(s: &String) {
    println!("{}", s);
}
