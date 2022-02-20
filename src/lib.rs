//! A collection of functions for string manipulation.

/// Removes the given character at the beginning and at the end of the string.
/// # Examples
/// ```
/// use kalgan_string;
///
/// assert_eq!(kalgan_string::strip("Hello World", 'H'), "ello World");
/// ```
/// ```
/// use kalgan_string;
///
/// assert_eq!(kalgan_string::strip("Hello World", 'd'), "Hello Worl");
/// ```
/// ```
/// use kalgan_string;
///
/// assert_eq!(kalgan_string::strip("-Hello World-", '-'), "Hello World");
/// ```
pub fn strip(string: &str, char: char) -> &str {
    let mut chars = string.chars();
    if !string.is_empty() {
        if string.chars().next().unwrap() == char {
            chars.next();
        }
        if string.chars().last().unwrap() == char {
            chars.next_back();
        }
    }
    chars.as_str()
}
/// Removes the given character at the end of the string.
/// # Examples
/// ```
/// use kalgan_string;
///
/// assert_eq!(kalgan_string::strip_right("Hello World", 'd'), "Hello Worl");
/// ```
pub fn strip_right(string: &str, char: char) -> &str {
    let mut chars = string.chars();
    if !string.is_empty() {
        if string.chars().last().unwrap() == char {
            chars.next_back();
        }
    }
    chars.as_str()
}
/// Removes the given character at the beginning of the string.
/// # Examples
/// ```
/// use kalgan_string;
///
/// assert_eq!(kalgan_string::strip_left("Hello World", 'H'), "ello World");
/// ```
pub fn strip_left(string: &str, char: char) -> &str {
    let mut chars = string.chars();
    if !string.is_empty() {
        if string.chars().next().unwrap() == char {
            chars.next();
        }
    }
    chars.as_str()
}
/// Removes the given characters at the beginning and at the end of the string.
/// # Examples
/// ```
/// use kalgan_string;
///
/// assert_eq!(kalgan_string::strip_both("Hello World", 'H', 'd'), "ello Worl");
/// ```
pub fn strip_both(string: &str, left: char, right: char) -> String {
    let mut chars = string.chars();
    if !string.is_empty() {
        if string.chars().next().unwrap() == left {
            chars.next();
        }
        if string.chars().last().unwrap() == right {
            chars.next_back();
        }
    }
    chars.as_str().to_string()
}
/// Checks whether the given string is a number.
/// # Examples
/// ```
/// use kalgan_string;
///
/// assert_eq!(kalgan_string::is_numeric("Hello World"), false);
/// ```
/// ```
/// use kalgan_string;
///
/// assert_eq!(kalgan_string::is_numeric("1.000"), true);
/// ```
pub fn is_numeric(string: &str) -> bool {
    match String::from(string).parse::<f64>() {
        Ok(_ok) => true,
        Err(_e) => false,
    }
}
