/*
Primitive Types in Rust
=====================
Integers: u8, u16, u32, u64, u128,, i128, usize (number of bits they take in memory)
floats: f32, f64
booleans: bool
characters: char (one character not a string)
tuples: Tuple structs can be used to group together data of different types. basically lists
arrays: fixed size, stack allocated
vectors are growable arrays
*/

pub fn run(){
    //default is i32
    let x = 1;

    //default is f64
    let y = 2.5;

    //Add explicit type
    let z:i64 = 453;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    //2147483647 number larger than that will need i64

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    let is_active = true;
    println!("{}", is_active);

    println!("{:?}", (is_active, x, y, z));

    //get boolean from expression
    let is_greater = 10 > 5;

    //chars are characterized by '' symbols
    let a1 = 'a';
    let face = '\u{1F600}'; //simley face as char
    println!("{:?}", (a1, face));

    println!("{}", is_greater);


}