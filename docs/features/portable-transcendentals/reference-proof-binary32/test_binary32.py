"""Exact embedding, asymmetric-cell and fail-closed controls for four power cases."""

import json
import unittest
from fractions import Fraction

from binary32_cells import binary32_cell, normal_word64, parse_binary32
from prove_binary32 import generate_binary32, prove_binary32
from verify_binary32 import HERE, verify_binary32, verify_binary32_record


class Binary32ProofControls(unittest.TestCase):
    def test_embedding_and_special_rejection(self) -> None:
        self.assertEqual(normal_word64(0x3f800000), 0x3ff0000000000000)
        self.assertEqual(normal_word64(0xbf800000), 0xbff0000000000000)
        self.assertEqual(normal_word64(0x3f000000), 0x3fe0000000000000)
        for word in (0, 1, 0x7f800000, 0x7fc00000, 0x100000000):
            with self.assertRaises(ValueError):
                normal_word64(word)

    def test_binary32_cells_and_negative_order(self) -> None:
        left, right = binary32_cell(0x3f800000)
        self.assertEqual(1 - left, Fraction(1, 2 ** 25))
        self.assertEqual(right - 1, Fraction(1, 2 ** 24))
        self.assertEqual(binary32_cell(0xbf800000), (-right, -left))

    def test_all_sources_and_enclosures_recompute(self) -> None:
        self.assertEqual(generate_binary32(), json.loads((HERE / 'enclosures.json').read_text()))
        verify_binary32()

    def test_adjacent_expected_word_fails(self) -> None:
        case = parse_binary32((HERE / 'correct-rounding-2b1f0e6.rs.txt').read_text())[0]
        with self.assertRaises(ValueError):
            prove_binary32(0, (case[0], case[1], case[2] + 1))

    def test_tampered_boundary_and_identity_fail(self) -> None:
        case = parse_binary32((HERE / 'correct-rounding-2b1f0e6.rs.txt').read_text())[0]
        record = prove_binary32(0, case)
        verify_binary32_record(record, case, 0)
        record['lower'] = record['cell_lower']
        with self.assertRaises(ValueError):
            verify_binary32_record(record, case, 0)
        with self.assertRaises(AssertionError):
            verify_binary32_record(record, case, 1)

    def test_wrong_source_cardinality_fails(self) -> None:
        with self.assertRaises(ValueError):
            parse_binary32('fn single_precision_pow_returns_the_correctly_rounded_reference()'
                           '{ let cases: = [(0x1, 0x2, 0x3)]; }')


if __name__ == '__main__':
    unittest.main()
