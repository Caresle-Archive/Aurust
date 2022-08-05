struct Point {
    x: i32,
    y: i32,
}

// Function that receives a reference to a struct Point
fn print_reference(r: &Point) {
    println!("x: {}, y: {}", r.x, r.y);
} 

fn main() {
    let x = Point { x: 30, y: 25 };
    
    // Creating a variable with a reference to a struct
    let y = &x;

    // Calling the function with x and y
    print_reference(&x);
    print_reference(&y);
}