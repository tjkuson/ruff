use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use rustpython_parser::ast::{Alias, Ranged, Stmt};

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for import statements that import a private name (a name starting
/// with an underscore `_`).
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
/// ## Known problems
/// Does not ignore private name imports from within the module that defines
/// the private name if the module is defined with [PEP 420] namespace packages
/// (directories that omit the `__init__.py` file).
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
/// [PEP 420]: https://www.python.org/dev/peps/pep-0420/
/// [`namespace-packages`]: https://beta.ruff.rs/docs/settings/#namespace-packages
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
    module_path: Option<&[String]>,
) {
    // Relative imports are not a public API, so we don't need to check them.
    if level.map_or(false, |level| level > 0) {
        return;
    }
    if let Some(module) = module {
        if module.starts_with("__future__") || module.starts_with("__main__") {
            return;
        }
        // Ignore private imports from the same module.
        // TODO(tjkuson): Make this work with PEP 420 namespace packages.
        if let Some(module_path) = module_path {
            let root_module = module_path.first().unwrap();
            if module.starts_with(root_module) {
                return;
            }
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
        for n in names {
            // It is common to import the package version as `__version__` and
            // to name translation functions `_`. Ignore these names.
            if matches!(n.name.as_str(), "__version__" | "_") {
                continue;
            }
            if n.name.starts_with('_') {
                checker.diagnostics.push(Diagnostic::new(
                    ImportPrivateName {
                        name: n.name.to_string(),
                    },
                    n.range(),
                ));
            }
        }
    }
}
