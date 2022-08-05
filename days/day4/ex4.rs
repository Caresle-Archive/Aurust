// Creating a function that recieve a mutable reference
fn change_name(word: &mut String) {
    word.push_str(" Nikola");
}

fn main() {
    let mut x = String::from("My awesome name");

    println!("{}", x);

    // Calling the function with the correct mutable reference
    change_name(&mut x);
    println!("{}", x);

}