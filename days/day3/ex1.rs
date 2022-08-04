
// Return an error message in case b equals to 0
fn division_with_zero(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("B cann't be 0"));
    }
    Ok(a / b)
}

fn match_result(r: Result<i32, String>) {
    match r {
        Ok(v) => println!("{}", v),
        Err(error) => println!("The error message say: {}", error),
    }
}

fn main() {
	// This will display an error message
    let r = division_with_zero(1, 0);
    match_result(r);

    // This will display the result of the division
    let r = division_with_zero(3, 1);
    match_result(r);

}