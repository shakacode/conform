use inflections::Inflect;

/// Removes leading and trailing whitespaces.
///
/// # Examples
///
/// ```
/// use conform::make::trim;
///
/// assert_eq!(trim("  foo  "), "foo".to_string());
/// ```
pub fn trim(string: &str) -> String {
  string.trim().to_string()
}

/// Removes leading whitespaces.
///
/// # Examples
///
/// ```
/// use conform::make::trim_left;
///
/// assert_eq!(trim_left("  foo  "), "foo  ".to_string());
/// ```
pub fn trim_left(string: &str) -> String {
  string.trim_left().to_string()
}

/// Removes trailing whitespaces.
///
/// # Examples
///
/// ```
/// use conform::make::trim_right;
///
/// assert_eq!(trim_right("  foo  "), "  foo".to_string());
/// ```
pub fn trim_right(string: &str) -> String {
  string.trim_right().to_string()
}

/// Converts any case into lower case ignoring separators.
///
/// # Examples
///
/// ```
/// use conform::make::lower;
///
/// assert_eq!(lower("Foo-Bar"), "foo-bar".to_string());
/// ```
pub fn lower(string: &str) -> String {
  string.to_lowercase()
}

/// Converts any case into UPPER CASE ignoring separators.
///
/// # Examples
///
/// ```
/// use conform::make::upper;
///
/// assert_eq!(upper("Foo-Bar"), "FOO-BAR".to_string());
/// ```
pub fn upper(string: &str) -> String {
  string.to_uppercase()
}

/// Converts any case into traditional sentence case without capitalizing the first letter.
///
/// # Examples
///
/// ```
/// use conform::make::sentence;
///
/// assert_eq!(sentence("Foo Bar"), "foo bar".to_string());
/// ```
pub fn sentence(string: &str) -> String {
  string.to_sentence_case()
}

/// Converts any case into title case where *every* word is capitalized.
///
/// # Examples
///
/// ```
/// use conform::make::title;
///
/// assert_eq!(title("foo bar"), "Foo Bar".to_string());
/// ```
pub fn title(string: &str) -> String {
  string.to_title_case()
}

/// Converts any case into camelCase.
///
/// # Examples
///
/// ```
/// use conform::make::camel;
///
/// assert_eq!(camel("foo bar"), "fooBar".to_string());
/// ```
pub fn camel(string: &str) -> String {
  string.to_camel_case()
}

/// Converts any case into PascalCase.
///
/// # Examples
///
/// ```
/// use conform::make::pascal;
///
/// assert_eq!(pascal("foo bar"), "FooBar".to_string());
/// ```
pub fn pascal(string: &str) -> String {
  string.to_pascal_case()
}

/// Converts any case into kebab-case.
///
/// # Examples
///
/// ```
/// use conform::make::kebab;
///
/// assert_eq!(kebab("Foo Bar"), "foo-bar".to_string());
/// ```
pub fn kebab(string: &str) -> String {
  string.to_kebab_case()
}

/// Converts any case into Train-Case.
///
/// # Examples
///
/// ```
/// use conform::make::train;
///
/// assert_eq!(train("foo bar"), "Foo-Bar".to_string());
/// ```
pub fn train(string: &str) -> String {
  string.to_train_case()
}

/// Converts any case into snake_case.
///
/// # Examples
///
/// ```
/// use conform::make::snake;
///
/// assert_eq!(snake("Foo Bar"), "foo_bar".to_string());
/// ```
pub fn snake(string: &str) -> String {
  string.to_snake_case()
}

/// Converts any case into CONSTANT_CASE.
///
/// # Examples
///
/// ```
/// use conform::make::constant;
///
/// assert_eq!(constant("Foo Bar"), "FOO_BAR".to_string());
/// ```
pub fn constant(string: &str) -> String {
  string.to_constant_case()
}
