pub mod compiler;
pub mod diagnostics;
mod semantic;
pub mod source;
pub mod syntax;

pub use semantic::{FunctionLookupError, TopLevelFunctionLookup, lookup_top_level_function};

pub const LANGUAGE_NAME: &str = "Cudrix";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_name_is_cudrix() {
        assert_eq!(LANGUAGE_NAME, "Cudrix");
    }
}
