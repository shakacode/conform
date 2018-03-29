extern crate conform;
#[macro_use]
extern crate conform_derive;
extern crate validator;
#[macro_use]
extern crate validator_derive;

use std::borrow::Cow;

use conform::Conform;
use validator::{Validate, ValidationError, ValidationErrors};

/// Integration with validate: validation success
#[test]
fn assert_integration_with_validate_success() {
  #[derive(Conform, Validate)]
  struct Subject {
    #[conform(trim, lower)]
    #[validate(email)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "  Foo@bar.com  ".to_string(),
  };

  assert_eq!(subject.conform().validate(), Ok(()));
}

/// Integration with validate: validation failure
#[test]
fn assert_integration_with_validate_failure() {
  #[derive(Conform, Validate)]
  struct Subject {
    #[conform(trim, lower)]
    #[validate(email)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "  Foobar.com  ".to_string(),
  };

  let mut error = ValidationError::new("email");
  error.add_param(Cow::from("value"), &"foobar.com");

  let mut errors = ValidationErrors::new();
  errors.add("prop", error);

  assert_eq!(subject.conform().validate(), Err(errors));
}

// TODO: `serde`
