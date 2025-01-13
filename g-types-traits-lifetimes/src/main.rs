struct Person {
    name: String,
    age: u8,
}
impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

trait Summary {
    fn summarize(&self) -> String;
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}


