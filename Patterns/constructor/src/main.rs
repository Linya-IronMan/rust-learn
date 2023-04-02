mod Person;
use Person::Person as Per;

fn main() {
    // let person1 = Per {
    //     name: "cdt".to_string(),
    //     age: 22,
    // };

    let default = Per::default();
    println!("Hello, world! {:?}", default);
}
