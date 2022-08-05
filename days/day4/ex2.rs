
// Function that receives a reference to a String
fn print_reference(r: &String) {
    println!("{}", r);
} 

fn main() {
    let x = String::from("Reference from x");
    let y = &x;

    // Calling the function with x and y
    print_reference(&x);
    print_reference(&y);
}