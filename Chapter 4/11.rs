/*Write a program to find all words starting with a “c” in a string passed as a parameter.
Concatenate them together and return the result.*/
fn words_starting_with_c(input: &str) -> String {
    input
        .split_whitespace()
        .filter(|word| word.to_lowercase().starts_with('c'))
        .collect::<String>()
}

fn main() {
    let text = "Cats climb cliffs carefully chasing clouds.";
    let result = words_starting_with_c(text);
    println!("Concatenated words starting with 'c': {}", result);
}
