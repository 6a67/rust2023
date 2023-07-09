mod structs {
    pub struct Person {
        name: String,
        age: u8,
    }

    impl Person {
        pub fn new(name: String, age: u8) -> Self {
            Person { name, age }
        }

        pub fn name(&self) -> &String {
            &self.name
        }

        pub fn age(&self) -> u8 {
            self.age
        }
    }
}

fn print_person(person: structs::Person) {
    println!("{} is {} years old.", person.name(), person.age(),);
}

fn main() {
    let person = structs::Person::new(String::from("Alice"), 20);

    match person {
        p @ structs::Person { .. } => {
            print_person(p);
        }
    }
}
