use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    count: usize,
    bound: Option<usize>,
    delims: (char, char),
}

impl<Iter> Progress<Iter> {
    fn new(iter: Iter) -> Self {
        Progress {
            iter,
            count: 0,
            bound: None,
            delims: ('[', ']'),
        }
    }
}

impl<Iter> Progress<Iter>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Progress<Iter> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.delims = delims;
        self
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);

        match self.bound {
            Some(bound) => {
                println!(
                    "{}{}{}{}",
                    self.delims.0,
                    "*".repeat(self.count),
                    " ".repeat(bound - self.count),
                    self.delims.1
                )
            }
            None => {
                println!("{}", "*".repeat(self.count))
            }
        }

        self.count += 1;

        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expected_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let brkts = ('<', '>');

    for n in (0..).progress().with_delims(brkts) {
        expected_calculation(&n);
    }

    let v = vec![1, 2, 3, 4, 5, 6];

    for n in v.iter().progress().with_bound().with_delims(brkts) {
        expected_calculation(n);
    }
}
