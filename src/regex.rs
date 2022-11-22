use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) struct StaticRegexes {
    pub(crate) line_comment: Regex,
    pub(crate) string_literal: Regex,
    pub(crate) assignment: Regex,
    pub(crate) say: Regex,
    pub(crate) the_answer_is: Regex,
    pub(crate) integer: Regex,
    pub(crate) decimal_float: Regex,
    pub(crate) ident: Regex,
    pub(crate) delim_front: Regex,
    pub(crate) delim_back: Regex,
    pub(crate) whitespace: Regex,
}

pub(crate) static RE: Lazy<StaticRegexes> = Lazy::new(|| {
    StaticRegexes {
        line_comment: Regex::new(r#""(.*)"#).unwrap(),
        string_literal: Regex::new(r#"<<(.*?)>>"#).unwrap(),
        assignment: Regex::new(r#"(should\s+be)"#).unwrap(),
        say: Regex::new(r#"(say)"#).unwrap(),
        the_answer_is: Regex::new(r#"(the\s+answer\s+is)"#).unwrap(),
        integer: Regex::new(r#"(-?[0-9](?:[0-9]|_?[0-9])*)"#).unwrap(),
        decimal_float: Regex::new(
            r#"(-?[0-9](?:[0-9]|_?[0-9])*\.[0-9](?:[0-9]|_?[0-9])*(?:e-?[0-9](?:[0-9]|_?[0-9])*)?)"#,
        )
        .unwrap(),
        ident: Regex::new(
            r#"([_\p{Sm}\p{Sk}\p{CurrencySymbol}\p{So}\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}][_\p{Sm}\p{CurrencySymbol}\p{Sk}\p{So}\p{Lu}\p{Ll}\p{Lt}\p{Lm}\p{Lo}\p{Nd}]*)"#,
        ).unwrap(),
        delim_front: Regex::new(r#"[\s"]$"#).unwrap(),
        delim_back: Regex::new(r#"^[\s"]"#).unwrap(),
        whitespace: Regex::new(r#"^\s+$"#).unwrap(),
    }
});
