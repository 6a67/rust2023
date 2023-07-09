struct Person {
    name: String,
    age: u8,
    height: f32,
}

fn main() {
    let height = 1.80;
    let person = Person {
        name: String::from("John"),
        age: 42,
        height,
    };
    println!("{} is {} years old.",
        person.name, person.age);
}