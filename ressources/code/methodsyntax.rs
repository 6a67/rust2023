impl Person {
    fn say_hello(&self) {
        println!("{} is {} years old.",
            self.name, self.age);
    }
}
