# Errors.
from _foo import bar
from foo._bar import baz
from _foo.bar import baz
from foo import _bar
from foo import _bar as bar

# Non-errors.
import foo
import foo as _foo
from foo import bar
from foo import bar as _bar
from foo.bar import baz
from foo.bar import baz as _baz
from .foo import _bar  # Relative import.
from .foo._bar import baz  # Relative import.
from ._foo.bar import baz  # Relative import.
from __future__ import annotations  # __future__ is a special case.
from __main__ import main  # __main__ is a special case.
from foo import __version__  # __version__ is a special case.
