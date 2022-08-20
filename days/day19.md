# Day 19

Another day of doing a few easy exercises of codechef

```rs
use std::io;

fn to_i32(numbers: &mut std::str::Split<&str>) -> i32 {
    let n = numbers
        .next()
        .unwrap()
        .trim()
        .parse::<i32>()
        .expect("Error parsing the number");
    n
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Error reading test cases");

    let t = buf.trim().parse::<i32>().expect("Error parsing test cases");

    for _ in 0..t {
        buf = String::new();

        stdin.read_line(&mut buf).expect("Error reading the three values");

        let mut split = buf.split(" ");
        let n = to_i32(&mut split);
        let x = to_i32(&mut split);
        let y = to_i32(&mut split);

        if (x + y * 2) <= n {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
```

```rs
use std::io;

fn to_i32(numbers: &mut std::str::Split<&str>) -> i32 {
    let n = numbers
        .next()
        .unwrap()
        .trim()
        .parse::<i32>()
        .expect("Error parsing the number");
    n
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Error parsing the test cases");

    let n = buf.trim().parse::<i32>().expect("Error parsing the number of cases");

    for _ in 0..n {
        buf = String::new();
        stdin.read_line(&mut buf).expect("Error reading the input value");

        let mut values = buf.split(" ");
        let k = to_i32(&mut values);
        let x = to_i32(&mut values);

        println!("{}", k * 7 - x);

    }
}
```