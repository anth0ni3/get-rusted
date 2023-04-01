struct Person {
    name: String,
    age: u8,
    likes_oranges: bool,
}

struct Point2D(u32, u32);

fn main() {
    let person = Person {
        name: String::from("John"),
        age: 25,
        likes_oranges: true,
    };

    println!("This person's name is {}", person.name);

    let origin = Point2D(100, 200);

    let Point2D(x, y) = origin;

    println!("Point contains:  {:?} and {:?}", x, y)
}
