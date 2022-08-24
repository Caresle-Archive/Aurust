# Day 23

Today i look at the rocket framework and did the hello world exercise.
Also complete two more challenges of codechef

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
        stdin.read_line(&mut buf).expect("Error reading the input");

        let mut splits = buf.split(" ");
        let alice = to_i32(&mut splits);
        let bob = to_i32(&mut splits);
        let charlie = to_i32(&mut splits);

        if alice > bob && alice > charlie {
            println!("Alice");
        } else if bob > alice && bob > charlie {
            println!("Bob");
        } else {
            println!("Charlie");
        }

    }
}
```


```rs
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Error parsing the test cases");

    let n = buf.trim().parse::<i32>().expect("Error parsing the number of cases");

    for _ in 0..n {
        buf = String::new();
        stdin.read_line(&mut buf).expect("Error reading the input");

        let hours = buf.trim().parse::<i32>().expect("Error parsing the number of hours");

        let games : i32 = (hours * 60) / 20;
        println!("{}", games);

    }
}
```