# Conform

[![crates.io version](https://img.shields.io/crates/v/conform.svg?style=flat-square)](https://crates.io/crates/conform)
[![build status](https://img.shields.io/travis/alexfedoseev/conform/master.svg?style=flat-square)](https://travis-ci.org/alexfedoseev/conform)
[![license](https://img.shields.io/crates/l/conform.svg?style=flat-square)](https://crates.io/crates/conform)

Rust macro to transform struct string fields in place.

Inspired by [`conform`](https://github.com/leebenson/conform) package for Go.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
conform = "x.x.x"
conform-derive = "x.x.x"
```

_where `x.x.x` is current version of crate._

## Usage
Example:

```rust
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
    email: "  Alex.Fedoseev@gmail.com".to_string(),
  };

  user.conform();

  println!("{:?}", user);
  // Prints: User { name: "Alex Fedoseev", email: "alex.fedoseev@gmail.com" }
}
```

`conform` works only on fields of `String` or `Option<String>` type.

`let` binding must be mutable.

### Usage with validator
You can use `conform` in conjunction with [`validator`](https://github.com/Keats/validator):

```rust
match user.conform().validate() {
  Ok(_) => /* save `user` to DB */,
  Err(err) => /* handle validation `err` */,
}
```

See [full example](./examples/src/main.rs).

### Transformations

#### `trim`
Removes leading and trailing whitespaces.

```
"  foo  " -> "foo"
```

#### `trim_left`
Removes leading whitespaces.

```
"  foo  " -> "foo  "
```

#### `trim_right`
Removes trailing whitespaces.

```
"  foo  " -> "  foo"
```

#### `lower`
Converts any case into lower case ignoring separators.

```
"Foo-Bar" -> "foo-bar"
```

#### `upper`
Converts any case into UPPER CASE ignoring separators.

```
"Foo-Bar" -> "FOO-BAR"
```

#### `sentence`
Converts any case into traditional sentence case without capitalizing the first letter.

```
"Foo Bar" -> "foo bar"
```

#### `title`
Converts any case into title case where *every* word is capitalized.

```
"foo bar" -> "Foo Bar"
```

#### `camel`
Converts any case into camelCase.

```
"foo bar" -> "fooBar"
```

#### `pascal`
Converts any case into PascalCase.

```
"foo bar" -> "FooBar"
```

#### `kebab`
Converts any case into kebab-case.

```
"Foo Bar" -> "foo-bar"
```

#### `train`
Converts any case into Train-Case.

```
"foo bar" -> "Foo-Bar"
```

#### `snake`
Converts any case into snake_case.

```
"Foo Bar" -> "foo_bar"
```

#### `constant`
Converts any case into CONSTANT_CASE.

```
"Foo Bar" -> "FOO_BAR"
```

## Licence
MIT.
