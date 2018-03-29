extern crate conform;
#[macro_use]
extern crate conform_derive;

use conform::Conform;

#[derive(Conform)]
struct Subject {
  #[conform(trim)]
  prop: Option<String>,
}

/// Some(v)
#[test]
fn assert_optional_some_string() {
  let mut subject = Subject {
    prop: Some("  Foo  ".to_string()),
  };

  subject.conform();

  assert_eq!(subject.prop, Some("Foo".to_string()));
}

/// None
#[test]
fn assert_optional_none() {
  let mut subject = Subject { prop: None };

  subject.conform();

  assert_eq!(subject.prop, None);
}
