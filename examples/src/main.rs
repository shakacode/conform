extern crate conform;
#[macro_use]
extern crate conform_derive;
extern crate validator;
#[macro_use]
extern crate validator_derive;

use conform::Conform;
use validator::Validate;

#[derive(Conform, Validate, Debug)]
struct User {
  #[conform(trim)]
  name: String,
  #[validate(email)]
  #[conform(trim, lower)]
  email: Option<String>,
}

fn main() {
  let mut user = User {
    name: "  Alex Fedoseev  ".to_string(),
    email: Some("  Alex.Fedoseev@gmail.com".to_string()),
  };
  match user.conform().validate() {
    Ok(_) => println!("Ok: {:?}", user),
    Err(err) => println!("Err: {:?}", err),
  }
}
