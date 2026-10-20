"""Exact binary32 source decoding and rounding cells for issue #201."""

import re
import sys
from fractions import Fraction
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'reference-proof'))
from exact_cells import exact_word


def normal_word64(word: int) -> int:
    """Embed a normal binary32 exactly; example: 0x3f800000 becomes 1.0 binary64."""
    exponent = (word >> 23) & 255
    if not 0 <= word < (1 << 32) or not 0 < exponent < 255:
        raise ValueError(f'word={word:#x}: expected finite normal binary32')
    return ((word >> 31) << 63) | ((exponent + 896) << 52) | ((word & 0x7fffff) << 29)


def binary32_cell(word: int) -> tuple[Fraction, Fraction]:
    """Use actual binary32 neighbors, not the much narrower embedded binary64 cell."""
    center = exact_word(normal_word64(word))
    previous, following = (word + 1, word - 1) if word >> 31 else (word - 1, word + 1)
    before = exact_word(normal_word64(previous))
    after = exact_word(normal_word64(following))
    return (before + center) / 2, (center + after) / 2


def parse_binary32(source: str) -> list[tuple[int, int, int]]:
    """Bind only the owned four-case function; example input is the archived source."""
    function = source.split('fn single_precision_pow_returns_the_correctly_rounded_reference()', 1)[1]
    literals = function.split('let cases:', 1)[1].split('];', 1)[0]
    triples = re.findall(r'\(\s*(0x[0-9a-f_]+),\s*(0x[0-9a-f_]+),\s*(0x[0-9a-f_]+)\s*,?\s*\)', literals)
    if len(triples) != 4:
        raise ValueError(f'case count={len(triples)}: expected four binary32 power records')
    return [tuple(int(word, 16) for word in triple) for triple in triples]
