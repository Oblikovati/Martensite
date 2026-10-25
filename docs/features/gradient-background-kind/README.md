# Retained gradient selection

Bug #305 corrected the active background selection made by
`set_gradient_background`. Previously the setter stored its colors and fill
method but left the active kind unchanged, so a rendering consumer could keep
showing a solid background.

The setter now selects the gradient kind after storing the supplied value.
A disabled fill still selects the gradient kind: the fill method determines
whether a gradient is drawn. A later explicit background-kind override remains
in force until the next gradient setter call. The solid background color is
independent and remains available when a fill is disabled.

The existing view interface and shared forwarding path retain this behavior.
Arguments and returned gradient values are snapshots, so editing either copy
does not mutate a previously stored gradient. No new rendering backend or
public API is introduced.

Owned tests use the real null graphic driver and shared retained-view protocol.
They cover horizontal and vertical updates, disabled fill, explicit overrides,
solid-color isolation and argument/readback snapshot independence. The correction
ships as a bug fix; selecting a rendering pass does not establish support for
all gradient fill methods in a particular backend.
