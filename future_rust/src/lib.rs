pub mod fs {
    use std::fs::File;
    use std::io::{Error, Read};
    use std::path::Path;

    #[deprecated(since = "0.4.0", note = "use std::fs::read_to_string")]
    pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
        let mut file = File::open(path)?;
        let buf_size = file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0);
        let mut string = String::with_capacity(buf_size);
        file.read_to_string(&mut string)?;
        Ok(string)
    }
}

// TODO: Deprecated in Rust 1.27+ (https://github.com/rust-lang/rust/issues/33417)
pub mod convert {
    pub trait TryFrom<T>: Sized {
        type Error;
        fn try_from(_: T) -> Result<Self, Self::Error>;
    }

    pub trait TryInto<T>: Sized {
        type Error;
        fn try_into(self) -> Result<T, Self::Error>;
    }
}

// TODO: Deprecated in Rust 1.26 (https://github.com/rust-lang/rust/issues/45860)
pub mod option {
    pub trait FilterExt<T> {
        fn filter_<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self;
    }

    impl<T> FilterExt<T> for Option<T> {
        // Copy-pasted from std
        #[inline]
        fn filter_<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self {
            if let Some(x) = self {
                if predicate(&x) {
                    return Some(x);
                }
            }
            None
        }
    }
}
