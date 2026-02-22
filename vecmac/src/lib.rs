use std::i32;

#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        $(vs.push($element);)*
        vs
    }}
}

trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn empty_vec() {
        max_impl!(i32);
        let x: Vec<u32> = avec![];
        assert!(x.is_empty())
    }

    #[test]
    fn single() {
        let x: Vec<u32> = avec![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn double() {
        let x: Vec<u32> = avec![42, 43, 45];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 3);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn trailing() {
        let x: Vec<&str> = avec![
            "dajdhqwjkdhqqjli",
            "djjhjqekHkh",
            "kjffhjkfaehfjkeh",
            "dAJfhqjhfq",
        ];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 4);
    }
}
