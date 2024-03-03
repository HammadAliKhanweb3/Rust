// clone function in Rust
// In Rust, the Clone trait allows you to create a copy of a value
// that has the same type and value but is a separate instance

fn main() {
    let original_string = String::from("Hello Pakistan");
    let clone_string = original_string.clone();

    println!("original string:{}", original_string);

    println!("clone string:{}", clone_string);

    let original_string = String::from("Original String");

    let modified_string = modify_string(&original_string);

    println!("original string:{}", original_string);

    println!("clone string:{}", modified_string);
}

fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone();
    cloned_string.push_str(" modified");
    cloned_string
}
