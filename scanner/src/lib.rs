use std::io::BufRead;

pub struct Scanner<B> {
    reader: B,
    buf_str: Vec<u8>,
    buf_iter: std::str::SplitWhitespace<'static>,
}
impl<B: BufRead> Scanner<B> {
    pub fn new(reader: B) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_whitespace(),
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> Option<T> {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return Some(token.parse().ok().expect("Failed parse"));
            }
            self.buf_str.clear();
            let z = self.reader.read_until(b'\n', &mut self.buf_str);

            if z.is_err() || z.unwrap() == 0 {
                return None;
            }

            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }

    pub fn next_split<T: std::str::FromStr>(&mut self) -> Vec<T> {
        loop {
            let mut v = Vec::new();
            while let Some(token) = self.buf_iter.next() {
                v.push(token.parse().ok().expect("Failed parse"));
            }
            if v.len() > 0 {
                return v;
            }

            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}
