struct Food {
    name: String,
    size: FoodSize
}

#[derive(Debug)]
enum FoodSize {
    Small,
    Big
}

fn main() {
    let f = Food {
        name: String::from("Apple"),
        size: FoodSize::Small
    };

    println!("name: {}, size: {:?}", f.name, f.size);

    let f = Food {
        name: String::from("Watermelon"),
        size: FoodSize::Big
    };
    println!("name: {}, size: {:?}", f.name, f.size);
}