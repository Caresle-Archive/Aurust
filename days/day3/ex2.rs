fn sum_negative(a: i32, b: i32) -> Result<i32, i32> {
    if a > 0 || b > 0 {
        return Err(-1);
    }

    if a == 0 && b == 0 {
        return Err(-2);
    }

    Ok(a + b)
}

fn match_result(r: Result<i32, i32>) {
    match r {
        Ok(v) => println!("The value is: {}", v),
        // Check if the error is '-1' and then write the next message in console
        Err(-1) => println!("The number need to be negative"),
        // Any error that isn't '-1'
        Err(e) => println!("Code error number {}", e),
    }
}

fn main() {
    // No error will throw
    let r = sum_negative(-1, -3);
    match_result(r);
    
    // Throw the error -1
    let r = sum_negative(1, 3);
    match_result(r);
    
    // Throw the error -2
    let r = sum_negative(0, 0);
    match_result(r);


}