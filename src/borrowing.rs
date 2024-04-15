
/**
 * Borrowing is a way to pass a reference
    * to a variable without changing ownership
**/
pub fn refer_length(s: &String) -> usize{
    /* if we try to modify something we don't own, rust prevents us */
    return s.len()
}
pub fn mut_reference(s: &mut String){
    s.push_str(" is the new value after mutating");
    let my_val = s;
    println!("The value of my_val is {}", my_val);
    // below, we can not access s anymore, it has been moved to my_val
    // let my_val1 = s;
    // but we can access my_val
    let my_val1 = my_val;
    println!("The value of my_val1 is {}", my_val1);
    // println!("The value of my_val1 is {} and its predecessor {}", my_val1, my_val);

}

// we cant have two mutable references to a variable at the same time

pub fn main_borrowing(){
    // idea of borrowing without ownership
    let str = String::from("hello world");
    let len = refer_length(&str);
    println!("The length of {} is {}", str, len);
    // we can also use mutable references
    let mut str = String::from("My muttable hello world");
    println!("Before mutation: {}", str);
    mut_reference(&mut str);
    println!("The new value of str is {}", str);
}