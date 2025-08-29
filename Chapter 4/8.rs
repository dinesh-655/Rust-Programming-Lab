/*Implement the string manipulation operations using Core Methods of String Objects
a. str.capacity()
b. str.contains("sub_str")
c. str.replace(replace_from, replace_to)
d. string.trim()*/
fn main() {
    let mut str = String::from("   Rust programming is fun   ");

    // a. Capacity
    println!("Capacity: {}", str.capacity());

    // b. Contains
    println!("Contains 'Rust': {}", str.contains("Rust"));

    // c. Replace
    let replaced = str.replace("fun", "awesome");
    println!("Replaced String: {}", replaced);

    // d. Trim
    let trimmed = str.trim();
    println!("Trimmed String: '{}'", trimmed);
}
