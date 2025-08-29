//Write a program to tokenize and iterate over a string
fn main() {
    let text = "Rust is fast and memory-efficient.";
    let tokens = text.split_whitespace();

    println!("Tokens:");
    for token in tokens {
        println!("{}", token);
    }
}
