//Write a program to push a string into a string.
fn main() {
    let mut base = String::from("Hello");
    let to_push = String::from(", world!");

    base.push_str(&to_push);

    println!("Concatenated String: {}", base);
}
