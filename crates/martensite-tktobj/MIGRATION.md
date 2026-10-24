# Object-model API migration

This crate is first published with the product-oriented `object_model` module and
its document, model, object, partition, iterator, attribute and type-registry
interfaces. The implementation had not appeared in a released package before this naming
review.

Consequently no released public identifier needs a compatibility alias and no
consumer migration is required for the first release. Future renames follow
Martensite's semantic-version policy: an additive replacement is introduced
alongside a deprecated earlier API for one release cycle; a removal is recorded
in the changelog and ships only in the appropriate breaking release.
