mod back_of_house;

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");

  meal.toast = String::from("Wheat");
  println!("{}", meal.toast);

  meal.seasonal_fruit = String::from("blueberries");

  let mut user = test::User::create("cdt");
  user.name = "123123".to_string();
}

mod test {
  pub struct User {
    pub name: String,
    age: u32,
  }
  impl User {
    pub fn create(name: &str) -> User {
      User {
        name: name.to_string(),
        age: 0,
      }
    }
  }
}
