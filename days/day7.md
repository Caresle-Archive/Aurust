# Day 7

## Resources

- [Implement generic types and traits](https://docs.microsoft.com/en-us/learn/modules/rust-generic-types-traits/)
- [Traits in Rust](https://youtu.be/T0Xfltu4h3A)
- [Traits in Rust Chapter 10.2 (Rust Book)](https://doc.rust-lang.org/book/ch10-02-traits.html)

## What i learn

I start learning about traist and how to implement them in rust.

Basic example of simple trait and his implementation
```rs
// Defining the trait Actor
trait Actor {
    fn show(&self) -> String;
}


// The struct for implementing the trait
struct Monster {
    name: String,
    hp: i32,
    atk: i32,
    def: i32,
}

// Implementation of the trait
impl Actor for Monster {
    fn show(&self) -> String {
        format!("My name is: {}", self.name)
    }
}

fn main() {
    let goblin = Monster {
        name: String::from("Green Goblin"),
        hp: 4,
        atk: 2,
        def: 1
    };
    println!("{}", goblin.show());
}
```

Another example of implementing a trait with a mutable reference
```rs
trait Actor {
    fn show(&self) -> String;
    // Function for change the hp of the actor
    fn take_damage(&mut self, damage: i32);
}

struct Monster {
    name: String,
    hp: i32,
    atk: i32,
    def: i32,
}

impl Actor for Monster {
    fn show(&self) -> String {
        format!("My name is: {}", self.name)
    }

    fn take_damage(&mut self, damage: i32) {
        // We need to deference the value before doing the operation
        *&mut self.hp -= damage;
    }
}

fn main() {
	// We change goblin to be mutable
    let mut goblin = Monster {
        name: String::from("Green Goblin"),
        hp: 4,
        atk: 2,
        def: 1
    };
    println!("{}", goblin.show());

    println!("Life: {}", goblin.hp);
    goblin.take_damage(2);
    println!("Life: {}", goblin.hp);
}
```

Example of a trait that implements a default behaviour
```rs
trait Actor {
    fn show(&self) -> String;
    fn take_damage(&mut self, damage: i32);
    fn goodbye(&self) -> String {
        format!("Default goodbye")
    }
}

struct Monster {
    name: String,
    hp: i32,
    atk: i32,
    def: i32,
}

impl Actor for Monster {
    fn show(&self) -> String {
        format!("My name is: {}", self.name)
    }

    fn take_damage(&mut self, damage: i32) {
        *&mut self.hp -= damage;
    }
}

fn main() {
    let mut goblin = Monster {
        name: String::from("Green Goblin"),
        hp: 4,
        atk: 2,
        def: 1
    };
    println!("{}", goblin.show());

    println!("Life: {}", goblin.hp);
    goblin.take_damage(2);
    println!("Life: {}", goblin.hp);

    println!("{}", goblin.goodbye());
}
```

Example of other struct implementing the Actor trait
```rs
trait Actor {
    fn show(&self) -> String;
    fn take_damage(&mut self, damage: i32);
    fn goodbye(&self) -> String {
        format!("Default goodbye")
    }
}

struct Monster {
    name: String,
    hp: i32,
    atk: i32,
    def: i32,
}

impl Actor for Monster {
    fn show(&self) -> String {
        format!("My name is: {}", self.name)
    }

    fn take_damage(&mut self, damage: i32) {
        *&mut self.hp -= damage;
    }
}

struct Player {
    name: String,
    hp: i32,
    magic: i32,
}

impl Actor for Player {
    fn show(&self) -> String {
        format!("My name is {} and my hp is: {} and my magic is: {}", self.name, self.hp, self.magic)
    }

    fn take_damage(&mut self, damage: i32) {
        *&mut self.hp -= damage;
    }

    fn goodbye(&self) -> String {
        format!("My special goodbye...")
    }
}

fn main() {
    let mut goblin = Monster {
        name: String::from("Green Goblin"),
        hp: 4,
        atk: 2,
        def: 1
    };
    println!("{}", goblin.show());

    println!("Life: {}", goblin.hp);
    goblin.take_damage(2);
    println!("Life: {}", goblin.hp);

    println!("{}", goblin.goodbye());

    let mut player = Player { name: String::from("Nikola"), hp: 10, magic: 20 };
    println!("{}", player.show());

    player.take_damage(5);

    println!("{}", player.show());
    println!("{}", player.goodbye());
}
```