#[macro_export]
macro_rules! avec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        $(vs.push($element);)*
        vs
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        let x = $element;
        for _ in 0..$count {
            vs.push(x.clone())
        }
        vs
    }}
}

#[cfg(test)]
mod test {

    #[test]
    fn empty_vec() {
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

    #[test]
    fn clone_2() {
        let x = avec![42; 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }

    #[test]
    fn clone_take() {
        let mut x = Some(42);
        let x = avec![x.take().unwrap(); 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }
}

///
/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;
