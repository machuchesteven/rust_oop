// this file deals with references and borrowing

fn get_str_len(s: String) -> (String, usize){
    let n = s.len();
    return (s, n);
}
fn ref_str_len(s: &String) -> usize{
    // if we try to modify something we don't own, rust prevents us
    s.len()
}
fn mutable_ref_len(s: &mut String){
    // restrictions on mutable references
    // we can only have one mutable reference to a variable
    // we can't create a further reference to the variable
    // avoids what we call prop drilling in react
    println!("\n\n About to mutate a variable\n\n");
    s.push_str(" is the new  after mutating value");
}

pub fn references(){
    // when we pass a variable to a function
    // once the function ends, the variable is dropped
    // but if we want to use the variable after we won't, hence let's see how to solve it
    let my_str = String::from("Mys string value");
    let (s, len) = get_str_len(my_str);
    // println!("The dropped value of my_str is {}", my_str);
    println!("The length of {} is {}", s, len);
    // we can also use the reference to the variable
    let my_new_str = String::from("Mys new string value");
    let (s, len) = get_str_len(my_new_str);
    // above we just pass the reference to the variable and the variable are not dropped
    //println!("The value expected to be dropped of my_new_str is {}", my_new_str); // this prevents it from running
    println!("The length of {} is {}", s, len);
    // we can also use the reference to the variable
    let my_new_str = String::from("Mys new string value");
    let len = ref_str_len(&my_new_str);
    // above we just pass the reference to the variable and the variable are not dropped
    println!("The length of {} is {}", my_new_str, len);
    // the & gives a reference without changing ownership
    // we can also use mutable references
    let mut my_new_str = String::from("Mys new string value");
    mutable_ref_len(&mut my_new_str);
    println!("The new value of my_new_str is {}", my_new_str);
}