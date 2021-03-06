use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) static RE_LINE_COMMENT: Lazy<Regex> = Lazy::new(|| Regex::new(r#""(.*)"#).unwrap());
pub(crate) static RE_STRING: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<<(.*?)>>"#).unwrap());
pub(crate) static RE_ASSIGN: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(should\s+be)"#).unwrap());
pub(crate) static RE_SAY: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(say)"#).unwrap());
pub(crate) static RE_THE_ANSWER_IS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(the\s+answer\s+is)"#).unwrap());
pub(crate) static RE_INTEGER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(-?[0-9](?:[0-9]|_?[0-9])*)"#).unwrap());
pub(crate) static RE_DECIMAL_FLOAT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"(-?[0-9](?:[0-9]|_?[0-9])*\.[0-9](?:[0-9]|_?[0-9])*(?:e-?[0-9](?:[0-9]|_?[0-9])*)?)"#,
    )
    .unwrap()
});
pub(crate) static RE_IDENT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"([_\p{Sm}\p{Sk}\p{CurrencySymbol}\p{So}\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}][_\p{Sm}\p{CurrencySymbol}\p{Sk}\p{So}\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nd}]*)"#,
    ).unwrap()
});
pub(crate) static RE_DELIM_FRONT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\s"]$"#).unwrap());
pub(crate) static RE_DELIM_BACK: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[\s"]"#).unwrap());
pub(crate) static RE_WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\s+$"#).unwrap());
