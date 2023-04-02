#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: &str, age: i32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            age: 0,
        }
    }
}
