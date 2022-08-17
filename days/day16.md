# Day 16

Today i have n't a lot of time, so i only complete the Second max of three numbers challenge
of CodeChef. Here is my solution.

```rs
use std::io;

// Convert the number into a i32
fn to_i32(numbers : &mut std::str::Split<&str>) -> i32 {
    let n = numbers
        .next()
        .unwrap()
        .trim()
        .parse::<i32>()
        .expect("Error reading the number");
    n
}

// Returns the index of the element to drop in the vector
fn max3(x: &i32, y: &i32, z: &i32) -> usize {
    if x > y && x > z {
        return 0usize;
    }

    if y > x && y > z {
        return 1usize;
    }

    2usize
}

fn max<'a>(x : &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    // Declaration of the input method and the buffer
    let stdin = io::stdin();
    let mut buf = String::new();

    // Read the number of cases
    stdin.read_line(&mut buf).expect("Error reading line");

    // Removing the end of the line and assigned the number
    // of cases to n
    let n = buf
        .trim()
        .parse::<i32>()
        .expect("Error parsing the number of cases");

    // Loop in all the cases declaration up
    for _ in 0..n {
        // Clear the buf
        buf = String::new();
        // Reading the numbers
        stdin
            .read_line(&mut buf)
            .expect("Error reading the numbers");
        // Split the numbres and converting them to i32
        let mut numbers = buf.split(" ");

        let n1 = to_i32(&mut numbers);
        let n2 = to_i32(&mut numbers);
        let n3 = to_i32(&mut numbers);
        let mut v = vec![n1, n2, n3];
        v.remove(max3(&n1, &n2, &n3));
        println!("{}", max(&v[0], &v[1]));

    }
}

```

It was a fun challenge, at the beginning i don't know how to handle the part of the second max number. Lucky for me, the problem says that `The three integers in a single triplet are all distinct`. With this i only need to write the function `max3` to get the index of the element to drop.