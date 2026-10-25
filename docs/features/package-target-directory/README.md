# Source archive output directory

Bug [#285](https://github.com/Oblikovati/Martensite/issues/285) stopped release
packaging after Cargo had verified the source archives. The release driver looked
under the checkout's `target/package` even when Cargo used an external target
directory.

The driver now retains `target_directory` from its existing Cargo metadata query.
It reads verified archives from that directory's `package` child. Cargo resolves
environment variables and configuration paths; the driver does not duplicate that
resolution or change the caller's environment. Archive license checks, checksum
creation, publication selection and immutable historical archives are unchanged.

Four real Cargo regression controls package a small dependency-free library with
the default directory, a relative configuration directory, and relative and
absolute environment directories. The relative environment control also overrides
a conflicting configuration value. Each control verifies the reported directory,
exact shipped archive bytes and checksum. Nondefault controls require that the
checkout's `target/package` was never created. A private named process adapter
sets each child environment; it never changes the parent process environment.
Missing or relative metadata output paths are rejected.

Before the fix, the default control passed and the three configured controls
failed at the old hard-coded lookup after actual Cargo package verification.
The fix changes archive selection, without retrying a build or moving its output.

The Cargo documentation describes the absolute
[`target_directory` metadata field](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)
and the different origins of
[environment and configuration paths](https://doc.rust-lang.org/cargo/reference/config.html#config-relative-paths).
