# Martensite topology test support

Development-only shape construction, profile and measurement fixtures shared by
Boolean and higher topology tests. This crate stays at the primitive/topological
construction dependency layer. It does not depend on Boolean, fillet, offset or
mesh toolkits. The existing foundation/geometry test-support crate remains lower.

These are the existing test helper implementations, including their original
assertions and numeric allowances. Moving them does not turn historical expected
values into independent mathematical evidence or certify integration error.
Boolean operation adapters stay with the tests that exercise those operations;
production modeling algorithms are reused through public kernel APIs.
