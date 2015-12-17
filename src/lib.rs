extern crate unicode_width;

use std::iter::Iterator;
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

#[cfg(test)] mod tests;

mod bytecount;
use bytecount::{ CharsExt, ByteCount };

struct PendingLine {
    text_offset: usize,
    line_start: usize,
}

pub struct WrappedLines<'a> {
    chars: ByteCount<'a>,
    max_width: usize,
    text: &'a str,
    pending: Option<PendingLine>,
}

pub trait LineWrapper {
    fn wrapped_lines(&self, width: usize) -> WrappedLines;
}

impl LineWrapper for str {

    fn wrapped_lines<'a>(&'a self, width: usize) -> WrappedLines<'a> {
        WrappedLines {
            chars: self.chars().byte_count(),
            max_width: width,
            text: self,
            pending: None,
        }
    }

}

impl<'a> Iterator for WrappedLines<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {

        let current_line;
        let mut width;

        match self.pending {
            Some(PendingLine { text_offset, line_start }) => {
                current_line = line_start;
                width = (&self.text[current_line..text_offset]).width();
            },
            None => loop {
                match self.chars.next() {
                    None => return None,
                    Some((_, chr)) if chr.is_whitespace() => (),
                    Some((n, chr)) => {
                        current_line = n;
                        width = chr.width().unwrap_or(1);
                        break;
                    },
                };
            },
        };

        let mut first_nonblank: Option<usize> = None;
        let mut last_word_end: Option<usize> = None;

        loop {

            let (text_offset, chr) = match self.chars.next() {
                Some((o, c)) => (o, c),
                None => {
                    self.pending = None;
                    return Some(&self.text[current_line..]);
                },
            };

            width += chr.width().unwrap_or(1);

            if chr.is_whitespace() {
                if first_nonblank != None {
                    last_word_end = Some(text_offset - 1);
                    first_nonblank = None;
                }
            } else {
                if first_nonblank == None {
                    first_nonblank = Some(text_offset);
                }
            }

            if width >= self.max_width {
                if let Some(lwe) = last_word_end {
                    self.pending = first_nonblank.map(|fnb| PendingLine { text_offset: text_offset, line_start: fnb });
                    return Some(&self.text[current_line..lwe+1]);
                }
            }

        }

    }

}
