"""Integer-only certificate verification, independent of the MPFR calculation."""

import hashlib
import json
from fractions import Fraction
from pathlib import Path

from binary32_cells import binary32_cell, normal_word64, parse_binary32
from exact_cells import rational_text

HERE = Path(__file__).resolve().parent


def verify_binary32_record(record: dict[str, object], case: tuple[int, int, int], index: int) -> None:
    """Recompute exact cell edges and reject boundary contact or incorrect identity."""
    first, second, expected = case
    assert record['index'] == index and record['expected'] == f'{expected:08x}'
    assert record['inputs'] == [f'{word:08x}' for word in (first, second)]
    assert record['embedded_binary64'] == [f'{normal_word64(word):016x}' for word in (first, second)]
    lower, upper = Fraction(record['lower']), Fraction(record['upper'])
    left, right = binary32_cell(expected)
    if not left < lower <= upper < right:
        raise ValueError(f'case={index}: expected strict binary32 rounding-cell inclusion')
    for key, value in (('cell_lower', left), ('cell_upper', right),
                       ('lower_margin', lower - left), ('upper_margin', right - upper)):
        assert record[key] == rational_text(value)


def verify_binary32() -> None:
    """Verify all four identities and integer certificates; run this script directly."""
    borrowed = json.loads((HERE / 'borrowed-source.json').read_text())
    for name, digest in borrowed['sha256'].items():
        assert hashlib.sha256((HERE.parent / borrowed['sibling'] / name).read_bytes()).hexdigest() == digest
    proof = json.loads((HERE / 'enclosures.json').read_text())
    cases = parse_binary32((HERE / 'correct-rounding-2b1f0e6.rs.txt').read_text())
    assert len(cases) == len(proof['cases']) == 4
    for name, digest in proof['source_sha256'].items():
        assert hashlib.sha256((HERE / name).read_bytes()).hexdigest() == digest
        assert parse_binary32((HERE / name).read_text()) == cases
    for index, (record, case) in enumerate(zip(proof['cases'], cases, strict=True)):
        verify_binary32_record(record, case, index)
    print('4 binary32 power identities and strict rational rounding cells verified')


if __name__ == '__main__':
    verify_binary32()
