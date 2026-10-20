"""Independent rounding certificates for valid neighbors of the #203 domains."""

import json
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'reference-proof'))
from exact_cells import rational_text, require_inside
from generate import directed
import gmpy2

CASES = (('acosh', 0x3ff0000000000001), ('atanh', 0x3fefffffffffffff),
         ('atanh', 0xbfefffffffffffff))


def certify_neighbor(name: str, argument: int) -> dict[str, object]:
    """Require strict MPFR enclosure inside one binary64 rounding cell."""
    lower = directed(name, (argument,), 256, gmpy2.RoundDown)
    upper = directed(name, (argument,), 256, gmpy2.RoundUp)
    expected = int.from_bytes(struct.pack('>d', float(lower)), 'big')
    require_inside(lower, upper, expected)
    return {'function': name, 'input': f'{argument:016x}', 'expected': f'{expected:016x}',
            'precision': 256, 'lower': rational_text(lower), 'upper': rational_text(upper)}


def certificates() -> list[dict[str, object]]:
    """Reproduce the three owned finite-neighbor references, without candidate calls."""
    result = [certify_neighbor(name, argument) for name, argument in CASES]
    assert len(result) == 3
    return result


if __name__ == '__main__':
    print(json.dumps(certificates(), indent=2, sort_keys=True))
