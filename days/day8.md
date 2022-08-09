# Day 8

## Resources

## What i learn

I learn how to pass a trait as a parameter like in the following code:
```rs
trait Actor {
    fn show(&self) -> String; 
}

struct Person {
    name: String,
}

struct Goblin {
    name: String,
    hp: i32
}

impl Actor for Person {
    fn show(&self) -> String {
        format!("My name is {}", self.name)
    }
}

impl Actor for Goblin {
    fn show(&self) -> String {
        format!("My name is {} and my hp is: {}", self.name, self.hp)
    }
}

// For passing a trait as parameter we need to use the impl and
// the trait name
fn print_actor(act: &impl Actor) {
    println!("{}", act.show());
}

fn main() {
    let gob = Goblin {
        name: String::from("Red Goblin"),
        hp: 20
    };
    let person = Person {
        name: String::from("Noa"),
    };

    print_actor(&gob);
    print_actor(&person);
}
```

And how to pass multiple traits in one parameter

```rs
// Add this to the previous code
trait Protagonist {
    fn hi(&self) -> String {
        format!("I'm a protagonist")
    }
}

impl Protagonist for Person {}


// We can use the '+' for asking for more traits as parameters
fn print_protagonist(act: &(impl Actor + Protagonist)) {
    println!("I'm an Actor {}, but also {}", act.show(), act.hi());
}

fn main() {
    // Add this to the end of the main body of the previous code
    print_protagonist(&person);
}
```

And now the `print_protagonist` function with the `trait bound syntax`
```rs
fn print_protagonist2<T: Actor + Protagonist>(act: &T) {
    println!("I'm an Actor {}, but also {}", act.show(), act.hi());
}
```

This function does the same, the only thing different is the syntax used for declaring it.
How it's explained in the rust book

> The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound

In the case that we need to define a lot of traits in one type we can use the `where` clause for this:

```rs
fn print_protagonist<T>(act: &T) 
    where T: Actor + Protagonist
{
    println!("I'm an Actor {}, but also {}\nAnd this function use the trait bount syntax", act.show(), act.hi());
}
```