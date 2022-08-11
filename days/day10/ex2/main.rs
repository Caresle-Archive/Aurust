mod vectors;

fn main() {
    let v = vectors::Vector2D {x: 10, y: 20};
    let v2 = vectors::Vector3D {x: 5, y: 15, z: 20};

    println!("v is smaller than v2? {}", vectors::get_small(&v, &v2));
}