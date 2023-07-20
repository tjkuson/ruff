use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use rustpython_parser::ast::{Alias, Ranged, Stmt};

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for import statements that import a private name.
///
/// ## Why is this bad?
/// [PEP 8] states that names starting with an underscore are private. Thus,
/// they are not intended to be used outside of the module in which they are
/// defined.
///
/// Further, as private imports are not considered part of the public API, they
/// are prone to unexpected changes, even in a minor version bump.
///
/// Instead, consider using the public API of the module.
///
/// ## Example
/// ```python
/// from foo import _bar
/// ```
///
/// ## References
/// - [PEP 8: Naming Conventions](https://peps.python.org/pep-0008/#naming-conventions)
/// - [Semantic Versioning](https://semver.org/)
///
/// [PEP 8]: https://www.python.org/dev/peps/pep-0008/
#[violation]
pub struct ImportPrivateName {
    name: String,
}

impl Violation for ImportPrivateName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ImportPrivateName { name } = self;
        format!("Imported private name `{name}`")
    }
}

/// PLC2701
pub(crate) fn import_private_name(
    checker: &mut Checker,
    stmt: &Stmt,
    names: &[Alias],
    module: Option<&str>,
    level: Option<u32>,
) {
    // Relative imports are not a public API, so we don't need to check them.
    if level.map_or(false, |level| level > 0) {
        return;
    }
    if let Some(module) = module {
        if module.starts_with("__future__") || module.starts_with("__main__") {
            return;
        }
        if module.starts_with('_') || module.contains("._") {
            let private_name = module
                .split('.')
                .find(|name| name.starts_with('_'))
                .unwrap_or(module);
            checker.diagnostics.push(Diagnostic::new(
                ImportPrivateName {
                    name: private_name.to_string(),
                },
                stmt.range(),
            ));
        }
        for name in names {
            if name.name.as_str() == "__version__" {
                continue;
            }
            if name.name.starts_with('_') {
                checker.diagnostics.push(Diagnostic::new(
                    ImportPrivateName {
                        name: name.name.to_string(),
                    },
                    name.range(),
                ));
            }
        }
    }
}
