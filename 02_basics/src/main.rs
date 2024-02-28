fn main() {
    
    // Variales
    //in rust varibales are immutable by default
    // for making variables mutable we have to use mut keyword

    // Array in Rust
    // mutable array of strings
    let mut hello: [String; 2] = [String::from("Hello"), String::from("World")];

    // changing value on 1 index
    hello[1] = String::from("Rust");

    // for strings we have to make colne of index when we access the index
    let second_element = hello[1].clone();
    println!("{}", second_element);

    // array of silces
    let days_of_weeks: [&str; 7] = [
        "Monday",
        "Tuesday",
        "wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "sunday",
    ];

    // getting first element of array
    let day_1 = days_of_weeks[0];
    println!("{}", day_1);

    // getting last element of array
    let last_num = days_of_weeks.len() - 1;
    println!("{}", last_num);

    // array of integers
    let numbers: [i32; 3] = [1, 2, 3];
    let number = numbers[1];
    println!("{}", number);

    let second_num = numbers[1];
    println!("{}", second_num);

    // Slices
    let slice = &days_of_weeks[1..3];
    let first_element_of_slice = slice[1];
    println!("{}", first_element_of_slice);

    // Tuples
    //Tuples in Rust are a data type that represents a fixed-size sequence of elements of different types
    let person = ("Hammad Ali Khan", 19);

    let name = person.0;
    let age = person.1;
    println!("Name of person is:{} and age is:{}", name, age);

    // Unit type
    // unit type in rust does'not return meaningfull value
    let _unit_type = ();
}
