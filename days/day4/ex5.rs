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

fn main() {
    let mut p1 = Point { x: 10, y: 20 };

    print_point(&p1);
    increase_x(&mut p1);
    print_point(&p1);

}