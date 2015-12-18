extern crate unicode_width;

use std::iter::Iterator;
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

#[cfg(test)]mod tests;

mod bytecount;
use bytecount::{CharsExt, ByteCount};

pub trait LineWrapper {
    fn wrapped_lines<'a>(&'a self, width: usize) -> WrappedLines<'a, DefaultSeparator>;
}

pub trait Separator {
    fn is_separator(&self, c: char) -> bool;
}

pub trait AsSeparator {
    type Inner: Separator;
    fn as_separator(self) -> Self::Inner;
}

pub struct DefaultSeparator;
pub struct ClosureSeparator(Box<Fn(char) -> bool>);

impl Separator for DefaultSeparator {
    #[inline]
    fn is_separator(&self, c: char) -> bool {
        c.is_whitespace()
    }
}

impl<F> AsSeparator for F where F: Fn(char) -> bool + 'static {

    type Inner = ClosureSeparator;

    #[inline]
    fn as_separator(self) -> ClosureSeparator {
        ClosureSeparator(Box::new(self))
    }

}

impl Separator for ClosureSeparator {
    #[inline]
    fn is_separator(&self, c: char) -> bool {
        self.0(c)
    }
}

impl LineWrapper for str {
    fn wrapped_lines<'a>(&'a self, width: usize) -> WrappedLines<'a, DefaultSeparator> {
        WrappedLines {
            chars: self.chars().byte_count(),
            max_width: width,
            text: self,
            pending: None,
            break_words: false,
            separator: DefaultSeparator,
        }
    }
}

struct PendingLine {
    text_offset: usize,
    line_start: usize,
}

pub struct WrappedLines<'a, S> {
    chars: ByteCount<'a>,
    max_width: usize,
    text: &'a str,
    pending: Option<PendingLine>,
    break_words: bool,
    separator: S,
}


impl<'a, S> WrappedLines<'a, S> {
    pub fn break_words(self, break_words: bool) -> Self {
        WrappedLines { break_words: break_words, ..self }
    }

    pub fn separator<F: AsSeparator>(self, sep: F) -> WrappedLines<'a, F::Inner> {
        WrappedLines {
            chars: self.chars,
            max_width: self.max_width,
            text: self.text,
            pending: self.pending,
            break_words: self.break_words,
            separator: sep.as_separator(),
        }
    }
}


impl<'a, S: Separator> Iterator for WrappedLines<'a, S> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {

        let current_line;
        let mut width;

        match self.pending {
            Some(PendingLine { text_offset, line_start }) => {
                current_line = line_start;
                width = (&self.text[current_line..(text_offset + 1)]).width();
            }
            None => {
                loop {
                    match self.chars.next() {
                        None => return None,
                        Some((_, chr)) if chr == '\n' || chr == '\r' => return Some(""),
                        Some((_, chr)) if self.separator.is_separator(chr) => (),
                        Some((n, chr)) => {
                            current_line = n;
                            width = chr.width().unwrap_or(1);
                            break;
                        }
                    };
                }
            }
        };

        let mut first_nonblank: Option<usize> = None;
        let mut last_word_end: Option<usize> = None;

        loop {

            let (text_offset, chr) = match self.chars.next() {
                Some((chr_offset, chr)) if chr == '\n' || chr == '\r' => {
                    self.pending = None;

                    let word_end;
                    if first_nonblank.is_some() {
                        word_end = chr_offset
                    } else {
                        word_end = last_word_end.unwrap_or(chr_offset) + 1
                    };
                    return Some(&self.text[current_line..word_end]);
                }
                Some((o, c)) => (o, c),
                None => {
                    self.pending = None;
                    return match (first_nonblank, last_word_end) {
                        (None, Some(n)) => Some(&self.text[current_line..(n + 1)]),
                        _ => Some(&self.text[current_line..]),
                    };
                }
            };

            width += chr.width().unwrap_or(1);

            if self.separator.is_separator(chr) {
                if first_nonblank != None {
                    last_word_end = Some(text_offset - 1);
                    first_nonblank = None;
                }
            } else {
                if first_nonblank == None {
                    first_nonblank = Some(text_offset);
                }
            }

            if width > self.max_width {
                let break_pos = match last_word_end {
                    Some(n) => Some(n + 1),
                    None if self.break_words => {
                        first_nonblank = Some(text_offset);
                        first_nonblank
                    }
                    None => None,
                };

                if let Some(pos) = break_pos {
                    self.pending = first_nonblank.map(|fnb| {
                        PendingLine {
                            text_offset: text_offset,
                            line_start: fnb,
                        }
                    });
                    return Some(&self.text[current_line..pos]);
                }
            }

        }

    }
}
