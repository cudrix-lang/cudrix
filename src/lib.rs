pub mod compiler;
pub mod diagnostics;
mod semantic;
pub mod source;
pub mod syntax;

pub use semantic::{
    DiscoveredDirectFunctionCall, DiscoveredIdentifierReference,
    EnclosingBlockLocalVariableCandidate, FunctionLookupError, FunctionParameterLookup,
    IdentifierReference, LocalVariableLookup, LocalVariableLookupError, ParameterLookupError,
    TopLevelFunctionLookup, discover_direct_function_calls, discover_identifier_references,
    lookup_direct_call_enclosing_block_local_variable_candidates,
    lookup_direct_call_enclosing_blocks, lookup_direct_call_enclosing_function_local_variables,
    lookup_direct_call_enclosing_function_parameters, lookup_direct_call_top_level_functions,
    lookup_identifier_reference_enclosing_blocks, lookup_top_level_function,
};

pub const LANGUAGE_NAME: &str = "Cudrix";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_name_is_cudrix() {
        assert_eq!(LANGUAGE_NAME, "Cudrix");
    }
}
