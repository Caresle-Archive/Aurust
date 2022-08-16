# Day 15

This day i continue with the rgb to hex project.

I split the projects as is show below

```
|- src/
  |- main.rs
  |- rgb.rs
```

**The project isn't complete, so be most of the code can change**

1. I decide to represent the RGB value into a struct.

```rs
pub struct RGB {
	pub r: i32,
	pub g: i32,
	pub b: i32,
}
```

2. Now that i have the `RGB` struct i create the next functions

```rs
impl RGB {
	pub fn to_hex(&self) {}
	fn get_value(&mut hex: String, target: i32) {}
}

```

The `to_hex` function is the container that will call all the methods and function necesary to convert an rgb value to hexadecimal.

The `get_value` is a function that convert a target value into a hexadecimal code and then add this code to the *hex* variable.