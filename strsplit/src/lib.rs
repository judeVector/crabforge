struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if self.delimiter.is_empty() {
            if remainder.is_empty() {
                return self.remainder.take();
            }
            let ch = &remainder[..1];
            *remainder = &remainder[1..];
            return Some(ch);
        }

        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }

    // fn next(&mut self) -> Option<Self::Item> {
    //     if let Some(ref mut remainder) = self.remainder {
    //         if let Some(next_delim) = remainder.find(self.delimiter) {
    //             let until_delimiter = &remainder[..next_delim];
    //             *remainder = &remainder[(next_delim + self.delimiter.len())..];
    //             Some(until_delimiter)
    //         } else {
    //             self.remainder.take()
    //         }
    //     } else {
    //         None
    //     }
    // }
}

#[cfg(test)]
mod it_works {
    use super::*;

    #[test]
    fn head() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(&haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(&haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }
}
