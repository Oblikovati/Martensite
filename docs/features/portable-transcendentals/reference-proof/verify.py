"""Check archived integer certificates; no high-precision dependency required."""

import hashlib
import json
from fractions import Fraction
from pathlib import Path

from exact_cells import exact_word, parse_cases, rational_text, require_inside, rounding_cell

HERE = Path(__file__).resolve().parent


def verify_record(record: dict[str, object], case: tuple[str, tuple[int, ...], int], index: int) -> None:
    """Bind identity and recompute both cell margins without floating arithmetic."""
    name, words, expected = case
    assert record['index'] == index and record['function'] == name
    assert record['inputs'] == [f'{word:016x}' for word in words]
    assert record['expected'] == f'{expected:016x}'
    assert record['input_rationals'] == [rational_text(exact_word(word)) for word in words]
    lower, upper = Fraction(record['lower']), Fraction(record['upper'])
    left, right = rounding_cell(expected)
    require_inside(lower, upper, expected)
    assert record['cell_lower'] == rational_text(left)
    assert record['cell_upper'] == rational_text(right)
    assert record['lower_margin'] == rational_text(lower - left)
    assert record['upper_margin'] == rational_text(right - upper)


def verify() -> None:
    """Verify all 95 records; run python3 verify.py from any directory."""
    proof = json.loads((HERE / 'enclosures.json').read_text())
    source = HERE / 'reference-2b1f0e6.rs.txt'
    cases = parse_cases(source.read_text())
    assert len(cases) == len(proof['cases']) == 95
    for name, digest in proof['source_sha256'].items():
        assert hashlib.sha256((HERE / name).read_bytes()).hexdigest() == digest
        assert parse_cases((HERE / name).read_text()) == cases
    for index, (record, case) in enumerate(zip(proof['cases'], cases, strict=True)):
        verify_record(record, case, index)
    print('95 exact identities and strict rational rounding-cell inclusions verified')


if __name__ == '__main__':
    verify()
