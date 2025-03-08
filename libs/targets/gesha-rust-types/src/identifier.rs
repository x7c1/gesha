use heck::ToUpperCamelCase;
use std::fmt::{Display, Formatter};
use syn::parse_str;
use syn::Ident;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct TypeIdentifier(String);

impl TypeIdentifier {
    pub fn generate<A: AsRef<str>>(a: A) -> Self {
        let a = a.as_ref();
        let converted = a.to_upper_camel_case();
        let result = parse_str::<Ident>(&converted);
        if result.is_ok() {
            return Self(converted);
        }
        let init: Vec<String> = vec!["".to_string()];
        let mut converted = a
            .chars()
            .fold(init, replace_symbol_with_name)
            .join("_")
            .to_upper_camel_case();

        if converted.starts_with(char::is_numeric) {
            converted = "_".to_string() + &converted;
        }
        // TODO: return error if incompatible chars found
        Self(converted)
    }
}

fn replace_symbol_with_name(mut acc: Vec<String>, c: char) -> Vec<String> {
    match ascii_symbol_to_name(c) {
        Some(converted) => {
            acc.push(converted.into());
            acc.push("".to_string());
        }
        _ => {
            let last = acc.len() - 1;
            acc[last].push(c);
        }
    };
    acc
}

impl Display for TypeIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl AsRef<str> for TypeIdentifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<TypeIdentifier> for String {
    fn from(this: TypeIdentifier) -> Self {
        this.0
    }
}

impl PartialEq<&str> for TypeIdentifier {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

fn ascii_symbol_to_name(c: char) -> Option<&'static str> {
    let str = match c {
        ' ' => "space",
        '!' => "exclamation",
        '"' => "double_quote",
        '#' => "hash",
        '$' => "dollar",
        '%' => "percent",
        '&' => "ampersand",
        '\'' => "apostrophe",
        '(' => "left_parenthesis",
        ')' => "right_parenthesis",
        '*' => "asterisk",
        '+' => "plus",
        ',' => "comma",
        '-' => "minus",
        '.' => "period",
        '/' => "slash",
        ':' => "colon",
        ';' => "semicolon",
        '<' => "less_than",
        '=' => "equals",
        '>' => "greater_than",
        '?' => "question",
        '@' => "at",
        '[' => "left_bracket",
        '\\' => "backslash",
        ']' => "right_bracket",
        '^' => "caret",
        '_' => "underscore",
        '`' => "backtick",
        '{' => "left_brace",
        '|' => "pipe",
        '}' => "right_brace",
        '~' => "tilde",
        _ => {
            // non-ascii character
            return None;
        }
    };
    Some(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_as_it_is() {
        let actual = TypeIdentifier::generate("hello_world");
        assert_eq!(actual, "HelloWorld");
    }

    #[test]
    fn ok_only_symbol() {
        let actual = TypeIdentifier::generate("*+-/");
        let expected = "AsteriskPlusMinusSlash";
        assert_eq!(actual, expected);
    }

    #[test]
    fn ok_starts_with_numeric() {
        let actual = TypeIdentifier::generate("123foo");
        let expected = "_123foo";
        assert_eq!(actual, expected);
    }

    #[test]
    fn ok_with_numeric_and_symbol() {
        let actual = TypeIdentifier::generate("1+foo=345%bar");
        let expected = "_1PlusFooEquals345PercentBar";
        assert_eq!(actual, expected);
    }
}
