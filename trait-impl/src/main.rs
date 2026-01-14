use std::{thread::sleep, time::Duration};

struct Progress<Iter> {
    iter: Iter,
    count: usize,
}

impl<Iter> Progress<Iter> {
    fn new(iter: Iter) -> Self {
        Progress { iter, count: 0 }
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("Printing {}", "*".repeat(self.count));
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
    let v = vec![1, 2, 3, 4, 5, 6];

    for n in v.iter().progress() {
        expected_calculation(n);
    }
}
