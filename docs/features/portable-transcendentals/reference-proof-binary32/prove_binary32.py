"""Four binary32 power certificates, reusing the binary64 MPFR evaluator (#201)."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
from binary32_cells import binary32_cell, normal_word64, parse_binary32
from exact_cells import rational_text
from generate import directed
import gmpy2


def prove_binary32(index: int, case: tuple[int, int, int]) -> dict[str, object]:
    """Prove the real power rounds directly to binary32, without double rounding."""
    first, second, expected = case
    words64 = (normal_word64(first), normal_word64(second))
    lower = directed('pow', words64, 256, gmpy2.RoundDown)
    upper = directed('pow', words64, 256, gmpy2.RoundUp)
    left, right = binary32_cell(expected)
    if not left < lower <= upper < right:
        raise ValueError(f'case={index}, expected={expected:#x}: enclosure must lie strictly inside binary32 cell')
    return {'index': index, 'inputs': [f'{word:08x}' for word in (first, second)],
            'expected': f'{expected:08x}', 'precision': 256,
            'embedded_binary64': [f'{word:016x}' for word in words64],
            'lower': rational_text(lower), 'upper': rational_text(upper),
            'cell_lower': rational_text(left), 'cell_upper': rational_text(right),
            'lower_margin': rational_text(lower - left), 'upper_margin': rational_text(right - upper)}


def generate_binary32() -> dict[str, object]:
    """Bind both immutable source revisions; run python3 prove_binary32.py."""
    paths = [HERE / f'correct-rounding-{pin}.rs.txt' for pin in ('2b1f0e6', '2e20418')]
    original, current = [parse_binary32(path.read_text()) for path in paths]
    if original != current:
        raise ValueError(f'cases={original},{current}: expected identical four binary32 records')
    return {'schema': 1, 'function': 'pow', 'format': 'binary32',
            'gmpy2': gmpy2.version(), 'mpfr': gmpy2.mpfr_version(),
            'source_sha256': {path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in paths},
            'cases': [prove_binary32(index, case) for index, case in enumerate(original)]}


if __name__ == '__main__':
    print(json.dumps(generate_binary32(), indent=2, sort_keys=True))
