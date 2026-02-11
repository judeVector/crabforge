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

    fn next(&mut self) -> Option<Self::Item> {}
}

#[cfg(test)]
mod it_works {
    use super::*;

    #[test]
    fn head() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(&haystack, "").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }
}
