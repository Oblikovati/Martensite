"""Certify the inverse gamma exponent and power without calling kernel code."""
import json
from pathlib import Path
import struct
import sys

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / 'reference-proof-binary32'))
from binary32_cells import binary32_cell, normal_word64
from exact_cells import exact_word, rational_text
from prove_binary32 import prove_binary32
import gmpy2


def inverse_exponent() -> dict[str, str]:
    """Prove the binary32 division result; inverse_exponent()['word'] is its bits."""
    reciprocal = 1 / exact_word(normal_word64(0x400CCCCD))
    left, right = binary32_cell(0x3EE8BA2E)
    if not left < reciprocal < right:
        raise ValueError(f'reciprocal={reciprocal}; expected strictly inside ({left},{right})')
    return {'word': '3ee8ba2e', 'exact_reciprocal': rational_text(reciprocal),
            'cell_lower': rational_text(left), 'cell_upper': rational_text(right)}


def rounded_power_word(exponent: int) -> int:
    """Round the independent real power directly at 24-bit precision."""
    exact = exact_word(normal_word64(exponent))
    with gmpy2.context(precision=256):
        argument = gmpy2.mpfr(gmpy2.mpq(exact.numerator, exact.denominator))
    with gmpy2.context(precision=24, round=gmpy2.RoundToNearest):
        result = gmpy2.mpfr('0.5') ** argument
    return struct.unpack('>I', struct.pack('>f', float(result)))[0]


def certificate() -> dict[str, object]:
    """Return exact division and directed power certificates; certificate()."""
    division = inverse_exponent()
    exponent = int(division['word'], 16)
    expected = rounded_power_word(exponent)
    return {'schema': 1, 'division': division,
            'power': prove_binary32(0, (0x3F000000, exponent, expected)),
            'gmpy2': gmpy2.version(), 'mpfr': gmpy2.mpfr_version()}


if __name__ == '__main__':
    print(json.dumps(certificate(), indent=2, sort_keys=True))
