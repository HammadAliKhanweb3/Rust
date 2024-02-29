fn main() {
    let mut counter = 0;

    // while loop
    while counter <= 5 {
        println!("Number is {}", counter);
        counter += 1;
    }

    // for loop
    for number in 1..5 {
        println!("The value of number is {}", number);
    }

    // for loop on array
    // array
    let numbers: [i32; 5] = [3, 23, 3, 4, 5];

    // for loop to print elements in array
    for number in numbers {
        println!("The value of number is {}", number);
    }

    // loop

    counter = 0;
    loop {
        println!("Number is {}", counter);
        counter += 1;

        if counter == 6 {
            break;
        }
    }
}
