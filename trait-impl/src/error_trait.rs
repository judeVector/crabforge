trait ErrorExt {
    fn any_is<T>(&self) -> bool
    where
        T: std::error::Error + Send + Sync + 'static;

    fn any_downcast_ref<T>(&self) -> Option<&T>
    where
        T: std::error::Error + Send + Sync + 'static;
}

impl ErrorExt for anyhow::Error {
    fn any_is<T>(&self) -> bool
    where
        T: std::error::Error + Send + Sync + 'static,
    {
        self.is::<T>() || self.chain().any(|e| e.is::<T>())
    }

    fn any_downcast_ref<T>(&self) -> Option<&T>
    where
        T: std::error::Error + Send + Sync + 'static,
    {
        std::iter::empty()
            .chain(self.downcast_ref::<T>())
            .chain(self.chain().flat_map(|e| e.downcast_ref::<T>()))
            .next()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    use std::io;

    use super::*;

    #[test]
    fn any_is_works_for_context() {
        let error = Result::<(), _>::Err(io::Error::other("Test"))
            .context("Foobar")
            .unwrap_err();

        assert!(error.any_is::<io::Error>())
    }
}

fn main() {}
