// Basic example of life time
fn main() 
{ // Begin of the life time of this expression, we will call this one -> 'a
    // Life time of each variable explained
    
    // The life time of x is -> 'a because it's right inside
    // of the block 'a 
    let x : &i32;

    { // Begin of another life time, we will call this -> 'b
        let y = 10; // the life time of y is 'b
        x = &y;

        // This doesn't have problem because y is still alive
        println!("{}", x);
    } // End of the life time 'b

    // This will produce an error because y doesn't live enough
    // That's because 'b ends before this expression is called
    // println!("{}", x);

} // End of the life time 'a