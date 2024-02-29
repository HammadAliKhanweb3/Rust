fn main() {
    // variable
    let day = "sunday";

    // if else conditonal statement
    if day == "sunday" {
        println!("The race day");
    } else if day == "saturday" {
        println!("Qualifing Day");
    } else {
        println!("wait patiently")
    }

    let num = 5;

    // conditional match statment
    match num {
        1 => println!("This number is one "),
        2 => println!("This  number is second "),
        3 => println!("This  number is second "),
        _ => println!("The number is something else"),
    }

    // we have to return value when we are storing match statement inside a variable
    let result = match num {
        1 => "This number is one ",
        2 => "This  number is second ",
        5 => "This  number is fifth ",
        _ => "The number is something else",
    };

    println!("The result is {}", result);
}
