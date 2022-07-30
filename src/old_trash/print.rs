pub fn run() {
    //print to console
    println!("Hello from the print.rs file!");

    //curly braces are placeholder for the data thats after the comma!
    println!("{} days", 31);

    //basic formatting
    println!("{}, this is {}", "Alice", "Bob");

    //positional arguments (multiple placeholders)
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    //named arguments
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    //placeholder traits (binary und hey)
    println!("{} Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10, 10,);

    //placeholder for debug trait (multiple datatypes (tuple) in one placeholder)
    println!("{:?}", (12, true, "hello"));

    //Basic math also possible
    println!("10 + 10 = {}", 10 + 10);

}