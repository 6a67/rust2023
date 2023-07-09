mod my_module {
    pub struct Person {
        pub name: String,
        pub age: u32,
    }
    impl Person {
        pub fn get_name(&self) -> &String {
            &self.name
        }
    }

    pub struct Vector(f64, f64);    // these fields are private by default
    impl Vector {
        pub fn new(x: f64, y: f64) -> Self {
            Vector(x, y)
        }
        pub fn get_xy(&self) -> (f64, f64) {
            (self.0, self.1)
        }
    }
}
fn main() {
    let p = my_module::Person {
        name: String::from("John"),
        age: 32,
    };
    println!("Name: {}", p.get_name());

    let v = my_module::Vector::new(1.0, 2.0);
    println!("Vector: {:?}", v.get_xy());
}