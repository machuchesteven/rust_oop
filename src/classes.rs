

// Functions with a return type must be indicated that it will have a return type using
// the -> type before using the return statement
pub fn add(
    a:i32,
    b:i32
) -> i32{
    return a+b;
}

/// functions that prints all even numbers below a given value and bigger than 100 forexample
/// ```
/// print_even(10);
/// ```
pub fn print_even(mut val:i32){
    // here we will use the while loop
    while val > 0 {
        if val % 2 == 0 {
            println!("{}", val);
        }
        val -= 1;
    }
    println!("All even number printed");
}

// using the loop function in rust, that executes over and over again until it is told to stop

pub fn print_odd(mut a:i32){
    loop {
        if a == 0 {
            break;
        }
        if a % 2 != 0 {
            println!("{}", a);
        }
        a = a -1;
    }
    println!("The loop running completed");
}

pub struct Person {
    name : String,
}

impl Person{
    pub fn new(name: &str) -> Person{
        Person{
            name: name.to_string(),
        }
    }
    pub fn hello(&self){
        println!("Hello from {}", self.name);
    }
}


pub fn ownerships(){
    println!("Hello, world!");
    // add the two numbers first
    let summation = add(5, 6);
    println!("{}", summation);
    print_even(summation);
    print_odd(summation);
    println!("The value of summation now is {}", summation);
    let machu = Person::new("Machuche");
    machu.hello();
    let str_val = "the string that can't be muatated";
    println!("{}",str_val);
    let mut mystr = "the string that can be mutated";
    println!("{}",mystr);
    mystr = "The new value ";
    // rust avoiding the double free error by copying the pointer not data
    let s1 = String::from(mystr);
    println!("{} : the value for previous s1 is", mystr);
    println!("{} : new s1 value", s1);
    // copying heap data - we use clone
    let s2 = s1.clone();
    println!("{} : the value for previous s1 is", s1);
    println!("{} : new s2 value", s2);
    // for integers, it is different, such integers which have know values at compile time are stored in the stack
    let x = 5;
    let y = x;
    println!("{} : x -> x and {} : y -> y", x, y);
    // for integers that are not known at compile time, they are stored in the heap
    // traits and lifetimes are another concept in rust very important to look at
    // even if a string is used as a parameter for a function, after it gets dropped and hence can not be used later
    // types like integers, booleans, floats and characters are stored in the stack
    // also tuples if are comprised of only values are stored in the stack, otherwise they are stored in the heap
}