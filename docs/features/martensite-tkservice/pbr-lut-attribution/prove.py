"""MPFR certificate for the one binary32 sine behind the changed split-sum LUT value.

The certificate encloses sin(phi) for the exact binary32 argument at 256 bits, once
rounding toward negative and once toward positive infinity, and checks that the
enclosure lies strictly inside the round-to-nearest cell of the current result word
and outside the cell of the previous one. Run with gmpy2 (MPFR) installed:

    python3 prove.py > /tmp/certificate.json && cmp certificate.json /tmp/certificate.json
"""

from __future__ import annotations

import json
import struct
from fractions import Fraction

import gmpy2

PHI = 0x3E96CBE4
PREVIOUS = 0x3E94A032  # binary32 platform sinf, the value before the migration
CURRENT = 0x3E94A031  # correctly rounded binary64 sine, rounded once more to binary32
PRECISION = 256


def binary32(word: int) -> Fraction:
    """The exact value of a finite binary32 word."""
    return Fraction(struct.unpack("<f", struct.pack("<I", word))[0])


def cell(word: int) -> tuple[Fraction, Fraction]:
    """The open interval of reals that round to `word` (positive normal words only)."""
    value = binary32(word)
    return (value + binary32(word - 1)) / 2, (value + binary32(word + 1)) / 2


def directed_sine(rounding: int) -> Fraction:
    """sin(phi) at PRECISION bits with one directed rounding, as an exact rational."""
    context = gmpy2.context(precision=PRECISION, round=rounding)
    with gmpy2.context(context):
        argument = gmpy2.mpfr(binary32(PHI).numerator) / binary32(PHI).denominator
        return Fraction(*gmpy2.sin(argument).as_integer_ratio())


def certificate() -> dict[str, object]:
    """Enclose sin(phi) and place it in the binary32 rounding cells."""
    lower, upper = directed_sine(gmpy2.RoundDown), directed_sine(gmpy2.RoundUp)
    current_cell, previous_cell = cell(CURRENT), cell(PREVIOUS)
    inside_current = current_cell[0] < lower <= upper < current_cell[1]
    inside_previous = previous_cell[0] < upper and lower < previous_cell[1]
    if not inside_current or inside_previous:
        raise ValueError("sin(phi) must round to the current word and not to the previous one")
    return {
        "function": "sin", "format": "binary32", "precision": PRECISION,
        "gmpy2": gmpy2.version(), "mpfr": gmpy2.mpfr_version(),
        "argument": f"{PHI:08x}", "previous": f"{PREVIOUS:08x}", "current": f"{CURRENT:08x}",
        "lower": str(lower), "upper": str(upper),
        "current_cell": [str(bound) for bound in current_cell],
        "lower_margin": str(lower - current_cell[0]), "upper_margin": str(current_cell[1] - upper),
        "current_is_correctly_rounded": True, "previous_is_correctly_rounded": False,
    }


if __name__ == "__main__":
    print(json.dumps(certificate(), indent=2, sort_keys=True))
