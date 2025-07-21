//4. Write a program to implement Scope and Shadowing
fn main() {
    let x = 5;
    println!("Outer x = {}", x);

    {
        let x = 10; // shadowing outer x
        println!("Inner x = {}", x);
    }

    println!("Outer x after inner scope = {}", x);
}
