"""Exact binary64 parsing and open nearest-rounding cells (issue #201)."""

import re
from fractions import Fraction


CASE = re.compile(r'\(\s*"([a-z0-9]+)"\s*,\s*(0x[0-9a-f_]+)\s*,\s*'
                  r'(0x[0-9a-f_]+)\s*(?:,\s*(0x[0-9a-f_]+)\s*)?,?\s*\)')


def parse_cases(source: str) -> list[tuple[str, tuple[int, ...], int]]:
    """Read literal words, ignoring decimal comments; see test_proof.py."""
    rows = [(name, tuple(int(word, 16) for word in words if word))
            for name, *words in CASE.findall(source)]
    return [(name, words[:-1], words[-1]) for name, words in rows]


def exact_word(word: int) -> Fraction:
    """Decode a finite word as a rational; example: exact_word(0) == 0."""
    exponent, significand = (word >> 52) & 2047, word & ((1 << 52) - 1)
    if exponent == 2047 or not 0 <= word < (1 << 64):
        raise ValueError(f'word={word:#x}: expected finite binary64')
    power = exponent - 1075 if exponent else -1074
    significand += (1 << 52) if exponent else 0
    signed = -significand if word >> 63 else significand
    return Fraction(signed) * Fraction(2) ** power


def rounding_cell(word: int) -> tuple[Fraction, Fraction]:
    """Return finite-neighbor midpoints; rejects zero/extremes explicitly."""
    center = exact_word(word)
    if center == 0 or abs(center) >= exact_word(0x7fefffffffffffff):
        raise ValueError(f'word={word:#x}: expected nonzero finite-neighbor case')
    previous, following = (word + 1, word - 1) if word >> 63 else (word - 1, word + 1)
    return (exact_word(previous) + center) / 2, (center + exact_word(following)) / 2


def rational_text(value: Fraction) -> str:
    """Serialize without rounding; example: Fraction(1, 2) becomes '1/2'."""
    return f'{value.numerator}/{value.denominator}'


def require_inside(lower: Fraction, upper: Fraction, word: int) -> None:
    """Reject even boundary contact; ties need a separate proof, never a guess."""
    left, right = rounding_cell(word)
    if not left < lower <= upper < right:
        raise ValueError(f'word={word:#x}: expected enclosure strictly inside rounding cell')
