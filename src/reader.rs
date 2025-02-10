struct LineReader<T: std::io::BufRead> {
    buf: T,
    backing_string: String,
}

trait LendingIterator<'a> {
    type Item;
    type Error: std::error::Error;

    fn next(&'a mut self) -> Option<Result<Self::Item, Self::Error>>;
}

// TODO: Implement the LendingIterator trait for LineReader
impl<'a, T: std::io::BufRead> LendingIterator<'a> for LineReader<T> {
    type Item = &'a str;
    type Error = std::io::Error;

    fn next(&'a mut self) -> Option<Result<Self::Item, Self::Error>> {
        self.backing_string.clear();
        match self.buf.read_line(&mut self.backing_string) {
            Ok(0) => None,
            Ok(_) => Some(Ok(self.backing_string.trim_end())),
            Err(e) => Some(Err(e)),
        }
    }
}


mod tests {
    use super::*;

    fn lines<T: std::io::BufRead>(reader: T) -> LineReader<T> {
        LineReader { buf: reader, backing_string: String::new() }
    }

    #[test]
    fn test_lines() {
        let reader = std::io::BufReader::new(&b"hello\nworld"[..]);
        
        let mut lines = lines(reader);
        assert_eq!(lines.next().unwrap().unwrap(), "hello");
        assert_eq!(lines.next().unwrap().unwrap(), "world");
    }
}