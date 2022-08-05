struct Point {
    x: i32,
    y: i32,
}

fn print_point(p: &Point) {
    println!("x: {}, y: {}", p.x, p.y);
}

fn increase_x(p: &mut Point) {
    p.x += 10;
}

// Function that returns a Result enum with a reference with a point
// in case nothing wron happened
fn point_positive(p: &Point) -> Result<&Point, String> {
    if p.x < 0 || p.y < 0 {
        return Err(String::from("Error, x or y are not positive"));
    }

    Ok(&p)
}

fn main() {
    let mut p1 = Point { x: 10, y: 20 };

    print_point(&p1);
    increase_x(&mut p1);

    let r = point_positive(&p1);
    match r {
        Ok(p) => print_point(p),
        Err(err) => println!("{}", err),
    }

    let p2 = Point {x: 30, y: -1};
    let r = point_positive(&p2);
    match r {
        Ok(p) => print_point(p),
        Err(err) => println!("{}", err),
    }

}