mod vectors {
    pub struct Vector2D {
        pub x: i32,
        pub y: i32,
    }

    pub struct Vector3D {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    pub trait VectorOp {
        fn sum_components(&self) -> i32;
    }

    impl VectorOp for Vector2D {
        fn sum_components(&self) -> i32 {
            self.x + self.y
        }
    }

    impl VectorOp for Vector3D {
        fn sum_components(&self) -> i32 {
            self.x + self.y + self.z
        }
    }

    // Return true if the vec is smaller than vec2
    // Otherwise return false
    pub fn get_small<T, U>(vec: &T, vec2: &U) -> bool 
    where T: VectorOp, 
    U: VectorOp {
        if vec.sum_components() < vec2.sum_components() {
            true
        } else {
            false
        }
    }
}

fn main() {
    let v = vectors::Vector2D {x: 10, y: 20};
    let v2 = vectors::Vector3D {x: 5, y: 15, z: 20};

    println!("v is smaller than v2? {}", vectors::get_small(&v, &v2));
}
