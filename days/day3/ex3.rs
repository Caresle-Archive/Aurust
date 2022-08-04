#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

fn check_age(person: Person) -> Result<Person, String> {
    if person.age < 0 {
        return Err(String::from("The age cannot be negative"));
    }

    // Returning a struct in case of nothing wron happend
    Ok(person)
}

fn match_result(r: Result<Person, String>) {
    match r {
        Ok(person) => println!("My name is {} and my age is {}", person.name, person.age),
        Err(error) => println!("{}", error),
    }
}

fn main() {
    let p1 = Person {
        name: String::from("Lucas"),
        age: 16
    };

    let p2 = Person {
        name: String::from("Lucas 2"),
        age: -1
    };

    // Ok
    let r = check_age(p1);
    match_result(r);

    // Err
    let r = check_age(p2);
    match_result(r);
}