use rustpython_parser::ast::{Ranged, Stmt};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};

/// ## What it does
/// Checks for `while` loops.
///
/// ## Why is this bad?
/// `while` loops are hard to read and understand. They are also prone to
/// infinite loops.
///
/// Often, code can be rewritten to use an alternative construct, such as
/// `for` loops and context managers. Exceptions to this rule are loops that
/// are intended to run indefinitely, such as event loops and listeners.
///
/// ## Example
/// ```python
/// i = n
/// while i > 0:
///     print(i)
///     i -= 1
/// ```
///
/// Use instead:
/// ```python
/// for i in range(n, 0, -1):
///     print(i)
/// ```
///
/// ## References
/// - [Python documentation](https://docs.python.org/3/reference/compound_stmts.html#the-while-statement)
/// - [Python documentation](https://docs.python.org/3/reference/compound_stmts.html#the-for-statement)
#[violation]
pub struct WhileLoop;

impl Violation for WhileLoop {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Used `while` loop")
    }
}

/// W0149
pub(crate) fn while_loop(stmt: &Stmt) -> Option<Diagnostic> {
    match stmt {
        Stmt::While(_) => Some(Diagnostic::new(WhileLoop, stmt.range())),
        _ => None,
    }
}
