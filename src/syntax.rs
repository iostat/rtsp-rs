use regex::Regex;

#[macro_use]
macro_rules! byte_map {
    ($($flag:expr,)*) => ([
        $($flag != 0,)*
    ])
}

lazy_static! {
    static ref QUOTED_STRING_RE: Regex =
        Regex::new(r#""((?:[ !#-\[\]-~]|[^[:ascii:]]|\\"|\\)*)""#).unwrap();
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const TOKEN_CHAR_MAP: [bool; 256] = byte_map![
 // 0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 1
    0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, // 2
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, // 3
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, // 5
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, // 7
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // C
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // D
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // E
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
];

pub fn is_token_char(b: u8) -> bool {
    TOKEN_CHAR_MAP[b as usize]
}

pub fn is_token<T>(token: T) -> bool
where
    T: AsRef<[u8]>,
{
    let bytes = token.as_ref();

    if bytes.is_empty() {
        return false;
    }

    for &b in bytes {
        if !is_token_char(b) {
            return false;
        }
    }

    true
}

pub fn quoted_string(string: &str) -> Option<&str> {
    QUOTED_STRING_RE
        .find(string)
        .map(|m| &string[m.start() + 1..m.end() - 1])
}

pub fn trim_whitespace(mut string: &str) -> &str {
    let mut old = 0;

    while string.len() != old {
        string = string.trim_right_matches("\r\n");
        string = string.trim_right_matches(|c| c == ' ' || c == '\t');
        old = string.len();
    }

    old = 0;

    while string.len() != old {
        string = string.trim_left_matches("\r\n");
        string = string.trim_left_matches(|c| c == ' ' || c == '\t');
        old = string.len();
    }

    string
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_token() {
        assert_eq!(true, is_token("this-is_a_~token~"));
        assert_eq!(true, is_token("token"));
        assert_eq!(false, is_token("not a token"));
    }

    #[test]
    fn test_quoted_string() {
        assert_eq!(None, quoted_string(r#""this is a test value"#));
        assert_eq!(
            Some("this is a test value"),
            quoted_string(r#""this is a test value""#)
        );
    }

    #[test]
    fn test_trim_whitespace() {
        assert_eq!(
            "this is a test value",
            trim_whitespace("this is a test value")
        );
        assert_eq!(
            "this is a test value",
            trim_whitespace("   this is a test value\t\r\n")
        );
        assert_eq!(
            "this is a test value\n",
            trim_whitespace("this is a test value\n")
        );
    }
}
