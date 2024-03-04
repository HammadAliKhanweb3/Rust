// vectors in rust
// vectors are like dynamic arrays means it can grow and shrink

fn main() {
    // vector with initalization
    let mut numbers = vec![1, 2, 3, 4];

    // pushing value in vector
    numbers.push(5);

    // for loop to access the elemens of vector
    for number in numbers {
        println!("{}", number);
    }

    // Another way to declare vector
    // empty vector
    let mut names: Vec<String> = Vec::new();

    names.push(String::from("Hammad Ali"));
    names.push(String::from("Khan"));

    let first_name = &names[0];
    let second_name = &names[1];

    println!("{}", first_name);
    println!("{}", second_name);

    // removig last element of vector
    names.pop();

    // getting slice of vector
    let _slice = &names[1..3];
}
