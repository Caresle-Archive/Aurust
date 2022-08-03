# Day 2

## Resources

The resources that i used for this day are the next:

- [Microsoft - Handle Errors in Rust](https://docs.microsoft.com/en-us/learn/modules/rust-error-handling/)
- [Rust Chapter 9 Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [A gentle introduction to results in rust](https://www.newline.co/@lsunsi/a-gentle-introduction-to-results-in-rust--b0391487)

## What i learn

I learn about how to prevent a program to panic with Option<T>.

Example: 
```rs
fn main() {
    let num = Some(9);
    // With match pattern
    match num {
        Some(10) => println!("The number is 10"),
        Some(n) => println!("The actual number is: {}", n),
        None => println!("There is no number")
    }
}
```

Example with *if let syntax*:
```rs
fn main() {
    let num = Some(9);

    match num {
        Some(n) => println!("The number is {}", n),
        None => {}
    }

    if let Some(n) = num {
        println!("The number is {}", n);
    }
}

```

# Problems

I have problems understanding how `Result` works. I have the basic idea that `Option` is for when we are looking for check the possibility of an absence value. While `Result` is the value of a operation that failed.

Most of my problems is how to return the correct value to `Result`.