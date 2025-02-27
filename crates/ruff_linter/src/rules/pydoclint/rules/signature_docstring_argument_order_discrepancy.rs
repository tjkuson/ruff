use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use crate::checkers::ast::Checker;

/// ## What it does
///
/// ## Why is this bad?
///
/// ## Example
/// ```python
/// ```
///
/// Use instead:
/// ```python
/// ```
#[derive(ViolationMetadata)]
pub(crate) struct SignatureDocstringArgumentOrderDiscrepancy;

impl Violation for SignatureDocstringArgumentOrderDiscrepancy {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("TODO: write message: {}", todo!("implement message"))
    }
}

/// DOC104
pub(crate) fn signature_docstring_argument_order_discrepancy(checker: &mut Checker) {}
