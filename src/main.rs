
fn main() {

// in rust if we are not using underscore before the name of variable without using it it will give warning  

// Boolean
let _my_myfirst_bool=false;

let my_second_bool=true;
println!("{my_second_bool}");
 
// Integers

// in rust we can specify the size of integer according to our need 
// 8 16 32 64 128
// by default it is 32

// 8 bit signed integer can hold both positive and negative values
let _my_integer=32;

// 128 bit integer
let _no_user:i128=20000;

// unsigned integer can hold only positive values it is best option for memory saving
let _no_tokens:u64=1000;

// Float

// flat values can have size 64 and 32 by default it is 64
let _pi:f32=3.14;

//Characters
// characters can store any unicode character
let  my_char: char = 'H';
println!("{}",my_char);

}
