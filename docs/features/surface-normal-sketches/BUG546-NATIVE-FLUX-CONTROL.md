# Native and conditional flux comparison (#546)

Bug [#546](https://github.com/Oblikovati/Martensite/issues/546) was found by running
the full public curved outer-contour and hole fixture with `NormalSense::Inward`.
The API completed under the original limits. The test then rejected a numerical
comparison that assumed a relationship absent from the result contract.

## The failed comparison

The q/L reference volume interval was
`[0.23758974858837012, 0.23758974858837015]`. The default native estimate was
`0.23758975356169265`, with raw native status `2.3092589893364427e-15`. The test
compared their separation, about `4.9733225e-9`, with
`proxy_reference_discrepancy + raw_native_status = 4.902686595582113e-9` and failed.
The original red log remains at
`/tmp/martensite-cone-public-inward-qualification-inward.log`.

The certified proxy is the signed occurrence flux of `F(p)=(p-apex)/3`.
Its reference discrepancy bounds only the difference between that flux and the
reference body volume. The native estimator integrates about a rough barycentre.
An exactly closed oriented surface has origin-independent flux, but the contract
does not promote tolerance-separated cap and side traces to exact geometric
closure. No certified native-to-proxy bridge supports the failed implication.

The raw status also has different units from an absolute volume discrepancy.
`BRepGProp::volume_properties_gk` divides its estimated absolute error by positive
volume above native `epsilon(1)`; otherwise it returns the absolute estimate.
The public `WrappedNativeVolumeEstimate.integration_error` retains that raw status.
Its documentation now states the convention. Stored values and legacy replay are
unchanged.

## Same-result observations

A disposable probe reused the completed inward result and its detached actual
oriented faces. Existing occurrence and full source-bit binding checks passed.
The certified apex flux was
`[0.23758975349105438, 0.2375897534910544]`. Native face quadrature at that same
apex gave `0.2375897534910536`, about `7.77e-16` outside the interval. The summed
native absolute face-error estimate was `6.75e-16`. Tightening the native relative
tolerance from `1e-8` to `1e-10` changed neither value nor estimate.

Changing only the quadrature origin to `[1,0,0]` changed the sum by about
`-7.366e-10`. The default native estimate differed from the apex sum by about
`+7.064e-11`. These observations support origin-sensitive patch flux. The probe
did not reconstruct the actual rough barycentre, so it does not quantitatively
attribute the entire default/apex difference to centring. All temporary probe
code was restored byte-for-byte before the permanent change.

## Corrected numerical control

The complete-operation test now sums the existing native GK face integrator over
the same exported oriented faces, at the original support apex. It compares that
numerical value with the certified boundary-flux interval. The target is the
unchanged requested native relative tolerance `1e-8` multiplied by the larger
absolute magnitude of the measured value and interval endpoints. There is no
extra absolute floor. Native estimated errors remain diagnostic estimates and do
not serve as rigorous bounds.

The native production call and this test share a private named constant for the
existing `1e-8` request. Its value, flags and production behavior are unchanged.
Independent primitive controls require a closed unit box to have flux one at
both origins, and a unit +X planar patch at x=1 to have flux `1/3` at zero and zero
at `[1,0,0]`. They check face completeness and origin sensitivity without another
full profile operation.

All authored/reference volume, storage, volume-policy, actual-use, normal,
achieved-bound, caller-history and isolation checks remain. The default native
estimate must remain finite and positive, and its raw status finite and
nonnegative. The new comparison is numerical agreement; it does not certify
native body volume or literal proxy closure. Focused qualification passed:
two primitive controls passed in 0.01 seconds, and both complete normal-sense
controls passed in 27.36 seconds. Their API times were 18.426147 seconds outward
and 8.842834 seconds inward while another build was running; these are not
performance acceptance measurements. Strict all-target Sketch Clippy passed.
The exit codes were all zero. Logs are
`/tmp/martensite-546-repair-qualification-primitives.log`,
`/tmp/martensite-546-repair-qualification-both-senses.log` and
`/tmp/martensite-546-repair-qualification-clippy.log`. The receipt records HEAD
`7678044cbd694477b26be17d48e7f6aa7af06586` with uncommitted feature work.
The final full Sketch suite remains pending. This change makes no Gate A, B or C
claim.
