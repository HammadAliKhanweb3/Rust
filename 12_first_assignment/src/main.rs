fn main() {
    // strings
    let string1 = String::from("Hammad Ali ");
    let string2 = String::from("Khan");

    // calling function and storing in a variable
    let concatenated_string = concatenate_strings(&string1, &string2);

    // printing result
    println!("{}", concatenated_string);
}

// function to concatenate two strings
fn concatenate_strings(str1: &String, str2: &String) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result
}
