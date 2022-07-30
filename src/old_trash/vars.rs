//variables hold primitive data or references to data
//variables are immutable by default
//variables can be mutable by using the mut keyword (constant by deafult) need to make it mutable to change it

pub fn run(){
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);
    
    age = 38;

    println!("My name is {} and i am {}", name, age);

    //define constant (all uppercase) -- not really needed in rust since theire immutable by default?
    //when using const have to specify type --> this is integer 32 bit
    const ID : i32 = 001;
    println!("ID: {}", ID);

    //assing multiple variables at once
    let (myname, myage) = ("Brad", 37);
    println!("{} is {}", myname, myage);
}