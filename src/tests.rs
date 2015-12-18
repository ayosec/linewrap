use super::LineWrapper;

#[test]
fn split_lipsum() {

    let text = "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu \
                fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in \
                culpa qui officia deserunt mollit anim id est laborum.";

    let mut lines = text.wrapped_lines(40).into_iter();
    assert_eq!(lines.next(), Some("Duis aute irure dolor in reprehenderit"));
    assert_eq!(lines.next(),
               Some("in voluptate velit esse cillum dolore eu"));
    assert_eq!(lines.next(), Some("fugiat nulla pariatur. Excepteur sint"));
    assert_eq!(lines.next(),
               Some("occaecat cupidatat non proident, sunt in"));
    assert_eq!(lines.next(), Some("culpa qui officia deserunt mollit anim"));
    assert_eq!(lines.next(), Some("id est laborum."));
    assert_eq!(lines.next(), None);
}

#[test]
fn bigger_chars() {

    let text = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
    let mut lines = text.wrapped_lines(10).into_iter();
    assert_eq!(lines.next(), Some("Ｈｅｌｌｏ,"));
    assert_eq!(lines.next(), Some("ｗｏｒｌｄ!"));
    assert_eq!(lines.next(), None);

}

#[test]
fn new_lines() {
    let text = "Duis aute irure dolor  \n in reprehenderit\n\n\nin voluptate velit esse cillum \
                dolore eu fugiat nulla pariatur.";

    let mut lines = text.wrapped_lines(40).into_iter();
    assert_eq!(lines.next(), Some("Duis aute irure dolor"));
    assert_eq!(lines.next(), Some("in reprehenderit"));
    assert_eq!(lines.next(), Some(""));
    assert_eq!(lines.next(), Some(""));
    assert_eq!(lines.next(),
               Some("in voluptate velit esse cillum dolore eu"));
    assert_eq!(lines.next(), Some("fugiat nulla pariatur."));
    assert_eq!(lines.next(), None);

}

#[test]
fn repeat_end() {
    let text = "  abcd def  ";
    let mut lines = text.wrapped_lines(100).into_iter();
    assert_eq!(lines.next(), Some("abcd def"));
    assert_eq!(lines.next(), None);
    assert_eq!(lines.next(), None);
    assert_eq!(lines.next(), None);
}

#[test]
fn keep_long_words() {

    let text = "pqr abcdefghijklmn opq rstu vwxyz";
    let mut lines = text.wrapped_lines(10).into_iter();
    assert_eq!(lines.next(), Some("pqr"));
    assert_eq!(lines.next(), Some("abcdefghijklmn"));
    assert_eq!(lines.next(), Some("opq rstu"));
    assert_eq!(lines.next(), Some("vwxyz"));
    assert_eq!(lines.next(), None);

}

#[test]
fn break_long_words() {

    let text = "pqr abcdefghijklmn opq rstu vwxyz";
    let mut lines = text.wrapped_lines(10).break_words(true).into_iter();
    assert_eq!(lines.next(), Some("pqr"));
    assert_eq!(lines.next(), Some("abcdefghij"));
    assert_eq!(lines.next(), Some("klmn opq"));
    assert_eq!(lines.next(), Some("rstu vwxyz"));
    assert_eq!(lines.next(), None);

}

#[test]
fn custom_separators() {

    let text = "abcdefg hijklm/nopqrstuvw xyz";

    let mut default_sep = text.wrapped_lines(20);
    assert_eq!(default_sep.next(), Some("abcdefg"));
    assert_eq!(default_sep.next(), Some("hijklm/nopqrstuvw"));
    assert_eq!(default_sep.next(), Some("xyz"));
    assert_eq!(default_sep.next(), None);

    let mut custom_sep = text.wrapped_lines(20).separator(|c: char| !c.is_alphanumeric());
    assert_eq!(custom_sep.next(), Some("abcdefg hijklm"));
    assert_eq!(custom_sep.next(), Some("nopqrstuvw xyz"));
    assert_eq!(custom_sep.next(), None);

}
