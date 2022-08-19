# Day 18

Today i complete 2 easy challenge of code chef.

Here is the code


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

    stdin.read_line(&mut buf).expect("Error reading number of test cases");

    let n = buf.trim().parse::<i32>().expect("Error parsing the number of cases");

    for _ in 0..n {
        buf = String::new();
        stdin.read_line(&mut buf).expect("Error reading the values of the case");

        let mut split = buf.split(" ");
        let number_students = to_i32(&mut split);
        let number_chairs = to_i32(&mut split);

        if number_students > number_chairs {
            println!("{}", number_students - number_chairs);
        } else {
            println!("0");
        }
    }
}
```

```rs
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Error reading test cases");

    let n = buf.trim().parse::<i32>().expect("Error parsing the number of cases");

    for _ in 0..n {
        buf = String::new();

        stdin.read_line(&mut buf).expect("Error reading the number of minutes");
        let n = buf.trim().parse::<i32>().expect("Error parsing the number");

        if n > 30 {
            println!("YES");
        } else {
            println!("NO");
        }

    }
}
```