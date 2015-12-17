use super::LineWrapper;

#[test]
fn split_lipsum() {

    let text = "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    let mut lines = text.wrapped_lines(40).into_iter();
    assert_eq!(lines.next(), Some("Duis aute irure dolor in reprehenderit"));
    assert_eq!(lines.next(), Some("in voluptate velit esse cillum dolore eu"));
    assert_eq!(lines.next(), Some("fugiat nulla pariatur. Excepteur sint"));
    assert_eq!(lines.next(), Some("occaecat cupidatat non proident, sunt in"));
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
