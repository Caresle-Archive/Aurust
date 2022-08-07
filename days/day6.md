# Day 6

## Resources

- [Implement generic types and traits](https://docs.microsoft.com/en-us/learn/modules/rust-generic-types-traits/)

## What i learn

This day, i spend most of my time learning about generic types and traits.

Basic example of generict type
```rs
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let boolean = Point {x: true, y: false};
    let integer = Point {x: 10, y: 20};
    let character = Point {x: 's', y: 'a'};

    println!("{:?}", boolean);
    println!("{:?}", integer);
    println!("{:?}", character);
}
```

Example of using more that 1 generic type for a struct
````rs
#[derive(Debug)]
// We add a 'U' parameter for specify another generic type.

// The type U can be different of T.
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let boolean = Point {x: true, y: 30};
    let integer = Point {x: 10, y: 'a'};
    let character = Point {x: 's', y: 'a'};

    println!("{:?}", boolean);
    println!("{:?}", integer);
    println!("{:?}", character);
}
```