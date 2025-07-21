/*2. Write a program to display output following pattern using placeholders
1
22
333
4444
55555*/
fn main() {
    for i in 1..=5 {
        for _ in 0..i {
            print!("{}", i);
        }
        println!();
    }
}
