# Day 11

## Resources

- [Rust test](https://docs.microsoft.com/en-us/learn/modules/rust-automated-tests/)

## What i learned

I learn about how to make test in rust.

**Basic example of a test**

```rs
fn add(a: i32, b: i32) -> i32 {
	a + b
}

#[test]
fn add_works() {
	assert_eq(add(1, 3), 3);
	assert_eq!(add(1, 4), 5);
}

#[test]
#[should_panic]
fn add_panics() {
	assert_eq!(add(1, 1), 3);
}

#[test]
#[ignore = "This test will be ignored"]
fn add_ignore() {}
```
