pub struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
    fn new(iter: O) -> Self {
        Flatten { outer: iter }
    }
}

pub fn flatten<O>(iter: O) -> Flatten<O> {
    Flatten::new(iter)
}

impl<O> Iterator for Flatten<O> {
    type Item = O::Item::Item;

    fn next(&mut self) -> Option<Self::Item> {}
}
