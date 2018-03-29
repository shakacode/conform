extern crate conform;
#[macro_use]
extern crate conform_derive;

use conform::Conform;

/// #[conform(trim)]
#[test]
fn assert_trim() {
  #[derive(Conform)]
  struct Subject {
    #[conform(trim)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "  Foo  ".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "Foo".to_string());
}

/// #[conform(trim_left)]
#[test]
fn assert_trim_left() {
  #[derive(Conform)]
  struct Subject {
    #[conform(trim_left)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "  Foo  ".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "Foo  ".to_string());
}

/// #[conform(trim_right)]
#[test]
fn assert_trim_right() {
  #[derive(Conform)]
  struct Subject {
    #[conform(trim_right)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "  Foo  ".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "  Foo".to_string());
}

/// #[conform(lower)]
#[test]
fn assert_lower() {
  #[derive(Conform)]
  struct Subject {
    #[conform(lower)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "Foo".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "foo".to_string());
}

/// #[conform(upper)]
#[test]
fn assert_upper() {
  #[derive(Conform)]
  struct Subject {
    #[conform(upper)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "Foo".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "FOO".to_string());
}

/// #[conform(sentence)]
#[test]
fn assert_sentence() {
  #[derive(Conform)]
  struct Subject {
    #[conform(sentence)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "Foo Bar".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "foo bar".to_string());
}

/// #[conform(title)]
#[test]
fn assert_title() {
  #[derive(Conform)]
  struct Subject {
    #[conform(title)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "foo bar".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "Foo Bar".to_string());
}

/// #[conform(camel)]
#[test]
fn assert_camel() {
  #[derive(Conform)]
  struct Subject {
    #[conform(camel)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "foo bar".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "fooBar".to_string());
}

/// #[conform(pascal)]
#[test]
fn assert_pascal() {
  #[derive(Conform)]
  struct Subject {
    #[conform(pascal)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "foo bar".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "FooBar".to_string());
}

/// #[conform(kebab)]
#[test]
fn assert_kebab() {
  #[derive(Conform)]
  struct Subject {
    #[conform(kebab)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "Foo Bar".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "foo-bar".to_string());
}

/// #[conform(train)]
#[test]
fn assert_train() {
  #[derive(Conform)]
  struct Subject {
    #[conform(train)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "foo bar".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "Foo-Bar".to_string());
}

/// #[conform(snake)]
#[test]
fn assert_snake() {
  #[derive(Conform)]
  struct Subject {
    #[conform(snake)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "foo bar".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "foo_bar".to_string());
}

/// #[conform(constant)]
#[test]
fn assert_constant() {
  #[derive(Conform)]
  struct Subject {
    #[conform(constant)]
    prop: String,
  }

  let mut subject = Subject {
    prop: "foo bar".to_string(),
  };

  subject.conform();

  assert_eq!(subject.prop, "FOO_BAR".to_string());
}
