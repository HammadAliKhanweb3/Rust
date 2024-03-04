fn main() {
    // string
    let mut my_string = String::from("South ");

    // Another way of declaring string
    let mut my_second_string = "Second string".to_string();

    // pushing string
    my_string.push_str("Africa");

    my_second_string.push_str(" in Rust");
    
    println!("{}", my_string);

    // iterating string by characters
    for c in my_second_string.chars() {
        println!("{}", c);
    }
    
    // iterating string by bytes
    for b in my_string.bytes() {
        println!("{}", b);
    }
}
