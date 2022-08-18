# Day 17

Today i complete the rgb to hexadecimal program. Here it's the final code:

> rgb.rs

```rs
pub struct RGB {
	pub r: i32,
	pub g: i32,
	pub b: i32,
}

const HEX_CODE : [char; 16] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];


impl RGB {
	pub fn to_hex(&self) {
		
		let mut hex = String::new();
		// Hexadecimal version of r, g & b
		let mut hex_r = Self::get_value(&mut hex, self.r);
		let mut hex_g = Self::get_value(&mut hex, self.g);
		let mut hex_b = Self::get_value(&mut hex, self.b);
		
		hex_r = Self::reverse(&mut hex_r);
		hex_g = Self::reverse(&mut hex_g);
		hex_b = Self::reverse(&mut hex_b);

		// Printing the color in hexadecimal
		println!("{}{}{}", hex_r, hex_g, hex_b);
	}

	fn get_value(hex: &mut String, target: i32) -> String {
		*hex = String::new();
		let mut value: i32 = 1;
		let mut rem : i32;
		let mut current_num = target;
		
		// Checking if the value passed is in the limits of rgb range
		if target >= 255 {
			return "FF".to_string();
		}

		if target <= 0 {
			return "00".to_string();
		}

		// Converting the number in case is in the range
		while value != 0 {
			value = current_num / 16;
			rem = current_num % 16;
			for (i, v) in HEX_CODE.iter().enumerate() {
				if i == (rem).try_into().unwrap() {
					hex.push(*v);
				}
			}
			
			current_num = value;
		}

		// Ading a 0 in case the number is less that 16
		if hex.len() < 2 {
			hex.push('0');
		}
		hex.to_string()
	}

	// Reversing the text to return the correct value
	// Example: rgb(15, 15, 15) 
	// Passed value F0 -> 240
	// return 0F -> 15 (Correct value)
	fn reverse(text : &mut str) -> String {
		let t = text.chars().rev().collect::<String>();
		t
	}
}
```

> main.rs

```rs
mod rgb;

use rgb::RGB;

fn main() {
    let color = RGB {r: 125, g: 125, b: 125};
    color.to_hex();

    let color = RGB {r: 125, g: 220, b: 130};
    color.to_hex();

    let color = RGB {r: 12, g: 24, b: 22};
    color.to_hex();

    let color = RGB {r: 0, g: 255, b: 125};
    color.to_hex();

    let color = RGB {r: -3000, g: 11111, b: 4};
    color.to_hex();
}
```