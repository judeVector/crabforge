pub struct Flatten<O>
where
    O: Iterator,
{
    outer: O,
    inner: Option<O::Item>,
}

impl<O> Flatten<O>
where
    O: Iterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
        }
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
{
    Flatten::new(iter)
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner_iter) = self.inner {
            if let Some(i) = inner_iter.next() {
                return Some(i);
            }
            self.inner = None;
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2);
    }
}
