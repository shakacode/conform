extern crate conform;
#[macro_use]
extern crate conform_derive;

use conform::Conform;

#[derive(Conform, Debug)]
struct User {
  #[conform(trim)]
  name: String,
  #[conform(trim, lower)]
  email: Option<String>,
}

fn main() {
  let mut user = User {
    name: "  Alex Fedoseev  ".to_string(),
    email: Some("  Alex.Fedoseev@gmail.com".to_string()),
  };
  user.conform();
  println!("{:?}", user);
}
