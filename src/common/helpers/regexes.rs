use regex::{Regex, RegexBuilder};

pub fn email_regex() -> Regex {
    Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]{2,}$").unwrap()
}

pub fn name_regex() -> Regex {
    RegexBuilder::new(r"(^[\p{L}0-9'\.\s]*$)")
        .unicode(true)
        .build()
        .unwrap()
}

pub fn jwt_regex() -> Regex {
    Regex::new(r"^[A-Za-z0-9-_=]+\.[A-Za-z0-9-_=]+\.?[A-Za-z0-9-_.+/=]*$").unwrap()
}

pub fn slug_regex() -> Regex {
    Regex::new(r"^[a-z\d]+(?:(\.|-)[a-z\d]+)*$").unwrap()
}

pub fn new_line_regex() -> Regex {
    Regex::new(r"\n").unwrap()
}

pub fn multi_spaces_regex() -> Regex {
    Regex::new(r"\s\s+").unwrap()
}
