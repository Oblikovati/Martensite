# Attribution of the changed split-sum LUT value

`crates/martensite-tkservice/tests/fixtures/service_corpus/native-03.txt`, line 134
(request `sv_pbr_lut 8.0 2.0 64.0`), changed one word: the second value of the first
texel moved from `4581620528485236736` to `4581620527948365824`, one binary32 unit in
the last place.

## Which call changed it

The LUT samples the GGX half vector with `sin` and `cos` of a binary32 angle `phi`. The
code before the migration called the binary32 platform functions (`phi.sin()`,
`phi.cos()`, which are glibc `sinf` and `cosf` on Linux). The crate now computes the
correctly rounded binary64 `sincos` of the same angle through `martensite-cmath` and
rounds each result once to binary32.

A temporary, uncommitted instrumentation of `lut_gen_importance_sample` compared both
paths on every call of the LUT corpus family (all seven requests, Linux x86-64, glibc
2.39). Exactly one call differs, and request 901 makes it:

| argument `phi` | binary32 platform `sinf` (before) | current path | `cos` (both paths) |
| --- | --- | --- | --- |
| `0x3e96cbe4` | `0x3e94a032` | `0x3e94a031` | `0x3f74fa0b` |

Replays of `service_corpus_replay::sv_pbr_lut` under the same instrumentation:

- binary32 platform `sinf`/`cosf` with the old fixture value: passes, so the old value
  comes back from the old calls;
- current path with the old fixture value: fails at request 901 only;
- current path, and current path with `martensite-cmath/host-libm`, with the new fixture
  value: pass.

The `host-libm` feature replays the platform binary64 functions; `martensite-cmath` has
no binary32 trigonometry, so `host-libm` cannot return the binary32 `sinf` result. The
platform binary64 sine of this argument, rounded to binary32, is the new word as well.

## The current word is the correctly rounded sine

`prove.py` encloses `sin(phi)` for the exact binary32 argument with MPFR at 256 bits,
rounding toward negative and toward positive infinity, and checks the enclosure against
the binary32 round-to-nearest cells. The enclosure lies strictly inside the cell of
`0x3e94a031`, so it is the correctly rounded binary32 sine, and outside the cell of
`0x3e94a032`, so the previous platform result was not. `certificate.json` keeps every
bound and margin; two runs give identical bytes:

```sh
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/tmp/martensite-reference-proof-python python3 prove.py > /tmp/certificate.json
cmp certificate.json /tmp/certificate.json
```

(gmpy2 2.3.2 with MPFR 4.2.2, installed as for `../../portable-transcendentals/reference-proof`.)

The changed LUT value is therefore attributed to one binary32 sine that the platform
rounded incorrectly and the current path rounds correctly (numerical behavior version 2).
