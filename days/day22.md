# Day 22

Today i only can complete one problem of codechef.


- [Problem](https://www.codechef.com/submit/EQUALDIST)

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
        let chocolates_alice = to_i32(&mut values);
        let chocolates_bob = to_i32(&mut values);

        if (chocolates_alice + chocolates_bob) % 2 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }

    }
}
```