fn main() {
    // Creating the variable wich will be reference later
    // This is a inmmutable reference
    let x = String::from("Hello");

    // Create a reference with `&` and the variable
    let y = &x;

    println!("{}", y);
}