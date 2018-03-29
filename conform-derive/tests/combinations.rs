extern crate conform;
#[macro_use]
extern crate conform_derive;

use conform::Conform;

/// All transformations are applied
#[test]
fn assert_all_conforms_are_applied() {
  #[derive(Conform)]
  struct Subject {
    #[conform(trim, upper)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "  Foo  ".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "FOO".to_string());
}

/// All transformations are applied in the correct order
#[test]
fn assert_all_conforms_are_applied_in_correct_order() {
  #[derive(Conform)]
  struct Subject {
    #[conform(lower, upper)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "Foo".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "FOO".to_string());
}
