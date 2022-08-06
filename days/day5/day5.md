# Day 5

## Resources
- [Rust Lifetimes Finally Explained!](https://youtu.be/juIINGuZyBc)
- [Understand how Rust manages memory](https://docs.microsoft.com/en-us/learn/modules/rust-memory-management/)
	- [Validate references by using lifetimes](https://docs.microsoft.com/en-us/learn/modules/rust-memory-management/3-validate-references-with-lifetimes)

## What i learn


I start learning about the lifetimes in rust. Also about how to declare this life times in
function, structures and variables. **Example of a function with all of this declaration**

```rs
fn longest<'a>(x: &'a String, y: &'a String) -> &'a String  {
	if x.len() > y.len() {
        x
    } else {
        y
    }
}

```