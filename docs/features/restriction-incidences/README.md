# Restriction endpoint incidences

Interval construction preserves each boundary incidence at an endpoint, including
records with the same spatial coordinate and different boundary ownership.
Coincident records keep their boundary metadata and take the selected endpoint's
line parameter. Their interval membership follows the selected endpoint.

The regression for #238 uses two endpoints with two distinct boundary arcs each.
It checks all four incidences after interval construction, their parameters and
membership, and source isolation. A second case checks the strict spatial
comparison, the larger endpoint tolerance, and parameter rebinding without
changing the boundary parameter. These expected results follow directly from
the declared incidence ownership and chosen interval endpoints.
