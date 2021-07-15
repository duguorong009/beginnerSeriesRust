struct Person {
    name: String,
    likes_oranges: bool, 
    age: u8,
}

struct Point2D(u32, u32);

fn main() {
    let person: Person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 23,
    };

    println!("Person name is {}", person.name);
    println!("Person likes oranges? {}", person.likes_oranges);
    println!("Person is {} years old", person.age);

    let point: Point2D = Point2D(32, 32);

    println!("the Point is {}, {}", point.0, point.1);
}
