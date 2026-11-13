"""Preserve the proof parser's grammar and bound malformed whitespace work (#441)."""

from pathlib import Path
import subprocess
import sys
import unittest

from binary32_cells import parse_binary32

PREFIX = 'fn single_precision_pow_returns_the_correctly_rounded_reference(){ let cases: = ['


def refuse_long_whitespace_record() -> None:
    """Exercise the pathological suffix in a child; `refuse_long_whitespace_record()`."""
    source = PREFIX + '(0x1,0x2,0x3' + ' ' * 1_000_000 + '!]; }'
    try:
        parse_binary32(source)
    except ValueError:
        return
    raise AssertionError('malformed record: expected a case-count refusal')


class Binary32ParserControls(unittest.TestCase):
    def test_whitespace_and_optional_commas_preserve_words(self) -> None:
        for tail in ('', ' ,', '\t,\n', '\n\t'):
            record = '(\n0x0_1,\t0x0_2,\n0x0_3' + tail + ')'
            self.assertEqual(parse_binary32(PREFIX + record * 4 + ']; }'), [(1, 2, 3)] * 4)

    def test_malformed_records_remain_refused(self) -> None:
        for record in ('(0x1 0x2,0x3)', '(0x1,0x2,0x3,,)', '(0x1,0x2,0xg)'):
            with self.assertRaises(ValueError):
                parse_binary32(PREFIX + record * 4 + ']; }')

    def test_long_whitespace_near_miss_finishes_within_budget(self) -> None:
        # A child bounds the old quadratic scan without platform-specific signal handling.
        worker = ('from test_binary32_parser import refuse_long_whitespace_record; '
                  'refuse_long_whitespace_record()')
        subprocess.run([sys.executable, '-B', '-c', worker],
                       cwd=Path(__file__).resolve().parent, capture_output=True,
                       text=True, check=True, timeout=5)


if __name__ == '__main__':
    unittest.main()
