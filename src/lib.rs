pub mod compiler;
pub mod diagnostics;
pub mod source;
pub mod syntax;

pub const LANGUAGE_NAME: &str = "Forge";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_name_is_forge() {
        assert_eq!(LANGUAGE_NAME, "Forge");
    }
}
