fn main() {
    // storing value
    let sum = add(2, 3);
    println!("Addition of two number is {}", sum);

    let _display = _display_message();
}

// function to add two numbers
fn add(a: i32, b: i32) -> i32 {
    let add: i32 = a + b;
    return add;
}

// function without parameters
fn _display_message() {
    println! {"This is just works"};
}
