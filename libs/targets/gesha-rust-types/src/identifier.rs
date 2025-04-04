use crate::ModuleName;
use gesha_core::conversions::Error::InvalidToken;
use gesha_core::conversions::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use std::fmt::{Display, Formatter};
use syn::Ident;
use syn::parse_str;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct TypeIdentifier(String);

impl TypeIdentifier {
    pub fn parse<A: AsRef<str>>(a: A) -> Result<Self> {
        let a = a.as_ref();
        let converted = a.to_upper_camel_case();
        let result = parse_str::<Ident>(&converted);
        if result.is_ok() {
            return Ok(Self(converted));
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
        if converted.is_empty() || !converted.is_ascii() {
            return Err(InvalidToken {
                target: a.to_string(),
            });
        }
        Ok(Self(converted))
    }

    pub fn to_mod_name(&self) -> ModuleName {
        ModuleName::new(self.0.to_snake_case())
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
    use rstest::rstest;

    #[rstest]
    #[case::as_it_is("hello_world", "HelloWorld")]
    #[case::only_symbol("*+-/", "AsteriskPlusMinusSlash")]
    #[case::only_symbol("123foo", "_123foo")]
    #[case::only_symbol("1+foo=345%bar", "_1PlusFooEquals345PercentBar")]
    #[case::with_minus("-42", "Minus42")]
    #[case::with_numeric_and_symbol("_42", "Underscore42")]
    #[case::with_symbol_and_numeric("%_42", "PercentUnderscore42")]
    fn ok(#[case] input: &str, #[case] expected: &str) {
        let actual = TypeIdentifier::parse(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::empty_string("")]
    #[case::non_ascii("🔥🔥🔥")]
    fn ng(#[case] input: &str) {
        let actual = match TypeIdentifier::parse(input) {
            Err(InvalidToken { target }) => target,
            other => panic!("expected error not returned but got: {other:?}"),
        };
        let expected = input.to_string();
        assert_eq!(actual, expected);
    }
}
