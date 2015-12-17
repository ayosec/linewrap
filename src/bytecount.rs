
use std::str::Chars;
use std::iter::Iterator;

pub trait CharsExt<'a> {
    fn byte_count(self) -> ByteCount<'a>;
}

impl<'a> CharsExt<'a> for Chars<'a> {
    fn byte_count(self) -> ByteCount<'a> {
        ByteCount {
            chars: self,
            byte: 0,
        }
    }
}

pub struct ByteCount<'a> {
    chars: Chars<'a>,
    byte: usize,
}

impl<'a> Iterator for ByteCount<'a> {
    type Item = (usize, char);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|c| {
            let r = (self.byte, c);
            self.byte += c.len_utf8();
            return r;
        })
    }
}
