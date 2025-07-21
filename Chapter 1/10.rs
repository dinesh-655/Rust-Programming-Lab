//10. Declare a String object and convert string literal to String object
fn main() {
    let s1 = String::from("Hello"); // from string literal to String object
    let s2 = "World".to_string();   // using to_string method

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
