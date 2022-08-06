
fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let w1 = String::from("Word 1");
    {
        let w2 = String::from("Word 33");
        let r = longest(&w1, &w2);
        println!("{}", r);
    }
}