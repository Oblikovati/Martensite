"""Reproduce directed MPFR enclosures, without importing candidate code."""

import hashlib
import json
from fractions import Fraction
from pathlib import Path

import gmpy2
from exact_cells import exact_word, parse_cases, rational_text, require_inside, rounding_cell


UNARY = {name: getattr(gmpy2, name) for name in (
    'acos acosh asin asinh atan atanh cbrt cos cosh exp exp2 expm1 '
    'log log10 log1p log2 sin sinh tan tanh').split()}
BINARY = {'atan2': gmpy2.atan2, 'hypot': gmpy2.hypot, 'pow': pow}
HERE = Path(__file__).resolve().parent


def mpfr_argument(word: int) -> gmpy2.mpfr:
    """Check exact conversion explicitly; example: the word for one maps to one."""
    exact = exact_word(word)
    converted = gmpy2.mpfr(gmpy2.mpq(exact.numerator, exact.denominator))
    numerator, denominator = converted.as_integer_ratio()
    if Fraction(int(numerator), int(denominator)) != exact:
        raise ValueError(f'word={word:#x}: expected exact MPFR input conversion')
    return converted


def directed(name: str, words: tuple[int, ...], precision: int, mode: int) -> Fraction:
    """MPFR's directed result bounds the exact function of exact dyadic inputs."""
    with gmpy2.context(precision=precision, round=mode, subnormalize=False,
                       emin=-1073741823, emax=1073741823):
        values = [mpfr_argument(word) for word in words]
        function = (UNARY if len(words) == 1 else BINARY)[name]
        result = function(*values)
        if not gmpy2.is_finite(result):
            raise ValueError(f'{name}{words}: expected finite MPFR result')
        numerator, denominator = result.as_integer_ratio()
        return Fraction(int(numerator), int(denominator))


def prove_case(index: int, case: tuple[str, tuple[int, ...], int]) -> dict[str, object]:
    """Accept only independently enclosed words; no candidate-derived fallback."""
    name, words, expected = case
    for word in (*words, expected):
        if not 0 < ((word >> 52) & 2047) < 2047:
            raise ValueError(f'word={word:#x}: expected scoped nonzero normal binary64')
    for precision in (256, 512, 1024, 2048):
        lower = directed(name, words, precision, gmpy2.RoundDown)
        upper = directed(name, words, precision, gmpy2.RoundUp)
        try:
            require_inside(lower, upper, expected)
        except ValueError:
            continue
        return record_case(index, case, precision, lower, upper)
    raise ValueError(f'case={index}, {name}{words}: no proven rounding cell at 2048 bits')


def record_case(index: int, case: tuple[str, tuple[int, ...], int], precision: int, lower: Fraction, upper: Fraction) -> dict[str, object]:
    """Keep exact endpoints and margins independently checkable with integers."""
    name, words, expected = case
    left, right = rounding_cell(expected)
    return {'index': index, 'function': name, 'inputs': [f'{word:016x}' for word in words],
            'expected': f'{expected:016x}', 'precision': precision,
            'input_rationals': [rational_text(exact_word(word)) for word in words],
            'lower': rational_text(lower), 'upper': rational_text(upper),
            'cell_lower': rational_text(left), 'cell_upper': rational_text(right),
            'lower_margin': rational_text(lower - left),
            'upper_margin': rational_text(right - upper)}


def generate() -> dict[str, object]:
    """Bind both source revisions and all cases; run as python3 generate.py."""
    paths = [HERE / name for name in ('reference-2b1f0e6.rs.txt', 'reference-2e20418.rs.txt')]
    cases, current = [parse_cases(path.read_text()) for path in paths]
    if cases != current or len(cases) != 95:
        raise ValueError(f'case counts={len(cases)},{len(current)}: expected identical 95 rows')
    return {'schema': 1, 'source_pins': ['2b1f0e6', '2e204184644a697823e5e1b8ac6ad2c608ce5e66'],
            'source_sha256': {path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in paths},
            'gmpy2': gmpy2.version(), 'mpfr': gmpy2.mpfr_version(), 'gmp': gmpy2.mp_version(),
            'rounding': ['MPFR_RNDD', 'MPFR_RNDU'],
            'cases': [prove_case(index, case) for index, case in enumerate(cases)]}


if __name__ == '__main__':
    print(json.dumps(generate(), sort_keys=True))
