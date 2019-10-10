use crate::prelude::*;

property!(
    /// `Name` is use for debugging purposes.
    #[derive(Default)]
    Name(String)
);

// --- Conversions ---

impl From<&str> for Name {
    fn from(s: &str) -> Name {
        Name(s.into())
    }
}
