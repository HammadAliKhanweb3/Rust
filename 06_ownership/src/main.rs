// ownership concept in rust
//  ** stack **
// in rust the stack is used to store values that have the known size at the compile time
// i.e premetive datatypes and local variables of function
// references of the varibles stores on stack that stored on the heap
// means we use stack as a map for accessing values on heap

//  ** heap **
// in rust the heap is used to store values that have the unknown size at the compile time
// i.e  strings and arrays etc

fn main() {
    // In rust there only one owner of the value at time by default

    // here S1 have the ownership of this String
    let s1 = String::from("Hammad Ali khan");

    // transfering ownership from s1 to s2
    let s2 = s1;

    // this line will give error because ownershiop has been transfered to s2
    //println!("{}", s1);
    println!("{}", s2);

    let x = 5;
    let y = String::from("how are you");
    let z = y;

    println!("value of x is:{} and value of z is {}", x, z);
}
