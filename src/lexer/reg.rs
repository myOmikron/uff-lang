use once_cell::sync::Lazy;
use regex::Regex;

pub static RE_LINE_COMMENT: Lazy<Regex> = Lazy::new(|| Regex::new(r#""(.*)"#).unwrap());
pub static RE_STRING: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(<<.*?>>)"#).unwrap());
pub static RE_ASSIGN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(should\s+be)"#).unwrap());
pub static RE_SAY: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(say)"#).unwrap());
pub static RE_INTEGER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(-?[0-9](?:[0-9]|_?[0-9])*)"#).unwrap());
pub static RE_DECIMAL_FLOAT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(-?[0-9](?:[0-9]|_?[0-9])*\.[0-9](?:[0-9]|_?[0-9])*(?:e-?[0-9](?:[0-9]|_?[0-9])*)?)"#,
    )
    .unwrap()
});
pub static RE_IDENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"([_\p{Sm}\p{Sk}\p{So}\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}][_\p{Sm}\p{Sk}\p{So}\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nd}]*)"#,
    ).unwrap()
});
pub static RE_DELIM_FRONT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\s"]$"#).unwrap());
pub static RE_DELIM_BACK: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\s"#).unwrap());
