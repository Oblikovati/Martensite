# Low-level evaluation boundary

## Decision and scope

The planar geometry capability in #105 preserves behavior version 1 of the
existing evaluator and adaptor APIs. These APIs return samples, derivatives or
low-level failures. They are not modelling operations and do not return the
completion, history, diagnostic context and validation coverage required for
publishing a modelling result.

This decision preserves existing return values and failure messages. It does not
certify those APIs as compliant modelling-operation result contracts. No checked
operation API, new numerical algorithm, validation result, or baseline revision
is introduced by this preparation.

## Empty grids

An empty sample array can represent several existing low-level conditions:

- no parameters were requested;
- no geometry was bound to a specialized evaluator;
- a derivative order below one was requested;
- offset evaluation could not produce its basis grid or normal.

For a B-spline or offset grid evaluator, inspect `geometry().is_null()` to
separate an unbound evaluator from one with a retained geometry handle. For a
nonempty request, check that the returned sample count covers the request.
Neither check establishes geometric validity. A zero-sample request does not
validate the geometry, and a bound handle does not prove that evaluation will
succeed. The retained empty-grid conventions must not be interpreted as successful
modelling completion or as an exact validation pass.

The regression tests cover both null evaluators and bound curves with empty
requests, plus nonempty requests to null evaluators and derivative orders from
minus one through four. They keep the existing result values unchanged.

## Adaptor failures

Undefined offset evaluation retains its existing failure category and exact
message for each derivative order. The fallback message identifies the evaluator
order; it does not contain the parameter, offset distance, affected entity, or
all conditions needed to explain the failure. Tests pin these exact categories
and messages so adding contextual reporting cannot silently change this contract.

A modelling operation using these evaluators must retain its request context and
return typed operation, entity and parameter diagnostics at its own boundary.
It must not parse these message strings to infer geometry identity or validity.
A missing or inconclusive required check remains incomplete at that boundary.
Changing the low-level messages or adding checked public evaluator entry points
requires a separately reviewed API and behavior decision; this capability does
neither.
